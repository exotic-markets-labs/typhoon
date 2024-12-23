use {
    accounts::{Account, Accounts},
    arguments::Arguments,
    injector::{FieldInjector, LifetimeInjector},
    proc_macro::TokenStream,
    quote::{format_ident, quote, ToTokens},
    remover::AttributeRemover,
    syn::{
        parse::Parse, parse_macro_input, parse_quote, spanned::Spanned, visit_mut::VisitMut,
        Generics, Ident, Item, Lifetime,
    },
};

mod accounts;
mod arguments;
mod constraints;
mod injector;
mod remover;

#[proc_macro_attribute]
pub fn context(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let context = parse_macro_input!(item as Context);

    TokenStream::from(context.into_token_stream())
}

struct Context {
    ident: Ident,
    generics: Generics,
    item: Item,
    accounts: Accounts,
    args: Option<Arguments>,
}
impl Parse for Context {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut item: Item = input.parse()?;
        LifetimeInjector.visit_item_mut(&mut item);

        match item {
            Item::Struct(mut item_struct) => {
                let args = item_struct
                    .attrs
                    .iter_mut()
                    .find(|attr| attr.meta.path().is_ident("args"))
                    .map(Arguments::try_from)
                    .transpose()?;

                // Remove the args attribute
                AttributeRemover::new("args").visit_item_struct_mut(&mut item_struct);

                let accounts = item_struct
                    .fields
                    .iter_mut()
                    .map(Account::try_from)
                    .collect::<Result<Vec<Account>, syn::Error>>()?;

                Ok(Context {
                    ident: item_struct.ident.to_owned(),
                    generics: item_struct.generics.to_owned(),
                    item: Item::Struct(item_struct),
                    accounts: Accounts(accounts),
                    args,
                })
            }
            Item::Enum(_item_enum) => todo!(), // TODO multiple context condition
            _ => Err(syn::Error::new(
                item.span(),
                "#[context] is only implemented for struct and enum",
            )),
        }
    }
}

impl ToTokens for Context {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let account_struct = &mut self.item.to_owned();
        let name = &self.ident;
        let generics = &self.generics;

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        let new_lifetime: Lifetime = parse_quote!('info);
        let (name_list, accounts_assign) = self.accounts.split_for_impl();
        let args_ident = format_ident!("args");

        let mut struct_fields = name_list.to_owned();

        let (args_struct, args_assign) = if let Some(ref args) = self.args {
            let name = args.get_name(name);

            FieldInjector::new(parse_quote! {
                pub args: Args<#new_lifetime, #name>
            })
            .visit_item_mut(account_struct);

            let args_struct = args.generate_struct(&name);
            let assign = quote! {
                let args = Args::<#name>::from_entrypoint(accounts, instruction_data)?;
            };

            struct_fields.push(&args_ident);

            (args_struct, Some(assign))
        } else {
            (None, None)
        };

        let expanded = quote! {
            #args_struct

            #account_struct

            impl #impl_generics HandlerContext<#new_lifetime> for #name #ty_generics #where_clause {
                fn from_entrypoint(
                    accounts: &mut &'info [typhoon_program::RawAccountInfo],
                    instruction_data: &mut &'info [u8],
                ) -> Result<Self, typhoon_program::program_error::ProgramError> {
                    let [#name_list, rem @ ..] = accounts else {
                        return Err(ProgramError::NotEnoughAccountKeys);
                    };

                    #args_assign
                    #accounts_assign

                    *accounts = rem;

                    Ok(#name { #(#struct_fields),*})
                }
            }
        };
        expanded.to_tokens(tokens);
    }
}
