use {
    crate::{
        constraints::{Constraint, Constraints},
        extractor::AccountExtractor,
    },
    proc_macro2::{Span, TokenStream},
    quote::{format_ident, quote, ToTokens},
    std::ops::Deref,
    syn::{
        parse_quote, spanned::Spanned, visit_mut::VisitMut, Field, Ident, PathSegment, Type,
        TypePath,
    },
};

#[derive(Clone)]
pub struct Account {
    pub(crate) name: Ident,
    pub(crate) constraints: Constraints,
    pub(crate) ty: PathSegment,
}

impl TryFrom<&mut Field> for Account {
    type Error = syn::Error;

    fn try_from(value: &mut Field) -> Result<Self, Self::Error> {
        let mut constraints = Constraints::default();
        constraints.visit_attributes_mut(&mut value.attrs);

        let segment = match &value.ty {
            Type::Path(TypePath { path, .. }) => path.segments.last(),
            _ => None,
        }
        .ok_or_else(|| syn::Error::new(value.span(), "Invalid type for the account"))?;

        let name = value
            .ident
            .clone()
            .unwrap_or(Ident::new("random", Span::call_site())); //TODO unit type

        Ok(Account {
            name,
            constraints,
            ty: segment.clone(),
        })
    }
}

pub struct NameList<'a>(Vec<&'a Ident>);

impl ToTokens for NameList<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let names = &self.0;
        let expanded = quote! {
            #(#names),*
        };

        expanded.to_tokens(tokens);
    }
}

impl<'a> Deref for NameList<'a> {
    type Target = Vec<&'a Ident>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Assign<'a>(Vec<(&'a Ident, &'a PathSegment, &'a Constraints)>);

impl ToTokens for Assign<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let assign_fields = self.0.iter().map(|(name, ty, c)| {
            if c.has_init() {
                let payer = c.get_payer();
                let space = c.get_space();

                let (Some(payer), Some(space)) = (payer, space) else {
                    return syn::Error::new(name.span(), "Not found payer or space for the init constraint").to_compile_error()
                };

                if let Some(punctuated_seeds) = c.get_seeds() {
                    quote! {
                        let #name: #ty = {
                            let system_acc = <typhoon::lib::Mut<typhoon::lib::SystemAccount> as typhoon::lib::FromAccountInfo>::try_from_info(#name)?;
                            // TODO: avoid reusing seeds here and in verifications
                            let bump = [bumps.#name];
                            let seeds = typhoon_program::seeds!(#punctuated_seeds, &bump);
                            let signer = typhoon_program::SignerSeeds::from(&seeds);
                            typhoon::lib::SystemCpi::create_account(system_acc, &#payer, &crate::ID, #space as u64, Some(&[signer]))?
                        };
                    }
                } else if c.is_seeded() {
                    let Some(keys) = c.get_keys() else {
                        return syn::Error::new(name.span(), "Seeded accounts require `keys` to be passed on init").to_compile_error()
                    };

                    let account_ty = AccountExtractor(ty).get_account_type();

                    quote! {
                        let #name: #ty = {
                            let system_acc = <typhoon::lib::Mut<typhoon::lib::SystemAccount> as typhoon::lib::FromAccountInfo>::try_from_info(#name)?;
                            // TODO: avoid reusing seeds here and in verifications
                            let bump = [bumps.#name];
                            let seeds = #account_ty::derive_with_bump(#keys, &bump);
                            let signer = typhoon_program::SignerSeeds::from(&seeds);
                            typhoon::lib::SystemCpi::create_account(system_acc, &#payer, &crate::ID, #space as u64, Some(&[signer]))?
                        };
                    }
                } else {
                    quote! {
                        let #name: #ty = {
                            let system_acc = <typhoon::lib::Mut<typhoon::lib::SystemAccount> as typhoon::lib::FromAccountInfo>::try_from_info(#name)?;
                            typhoon::lib::SystemCpi::create_account(system_acc, &#payer, &crate::ID, #space as u64, None)?
                        };
                    }
                }
            } else {
                quote! {
                    let #name = <#ty as FromAccountInfo>::try_from_info(#name)?;
                }
            }
        });

        let expanded = quote! {
            #(#assign_fields)*
        };
        expanded.to_tokens(tokens);
    }
}

pub struct AdditionalChecks<'a>(Vec<(&'a Ident, &'a Constraints)>);

impl ToTokens for AdditionalChecks<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut constraints = Vec::new();

        for (name, constraints_list) in &self.0 {
            let var_name = format_ident!("{}_state", name);
            let mut has_ones = Vec::new();

            for constraint in &constraints_list.0 {
                if let Constraint::HasOne(has_one) = constraint {
                    let target = &has_one.join_target;
                    let basic_error = parse_quote!(Error::HasOneConstraint);
                    let error = has_one.error.as_ref().unwrap_or(&basic_error);
                    has_ones.push(quote! {
                        if &#var_name.#target != #target.key() {
                            return Err(#error.into());
                        }
                    });
                }
            }

            if !has_ones.is_empty() {
                constraints.push(quote! {
                    {
                        let #var_name = #name.data()?;
                        #(#has_ones)*
                    }
                });
            }
        }

        let expanded = quote! {
            #(#constraints)*
        };

        expanded.to_tokens(tokens);
    }
}

pub struct Accounts(pub Vec<Account>);

impl Accounts {
    pub fn split_for_impl(&self) -> (NameList, Assign, AdditionalChecks) {
        let mut names = Vec::with_capacity(self.0.len());
        let mut assigns = Vec::with_capacity(self.0.len());
        let mut additional_checks = Vec::with_capacity(self.0.len());

        for el in &self.0 {
            names.push(&el.name);
            assigns.push((&el.name, &el.ty, &el.constraints));
            additional_checks.push((&el.name, &el.constraints));
        }

        (
            NameList(names),
            Assign(assigns),
            AdditionalChecks(additional_checks),
        )
    }
}
