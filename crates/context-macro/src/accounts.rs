use {
    crate::constraints::Constraints,
    proc_macro2::{Span, TokenStream},
    quote::{quote, ToTokens},
    std::ops::Deref,
    syn::{spanned::Spanned, visit_mut::VisitMut, Field, Ident, PathSegment, Type, TypePath},
};

pub struct Account {
    name: Ident,
    constraints: Constraints,
    ty: PathSegment,
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

                quote! {
                    let #name: #ty = {
                        let system_acc = <Mut<SystemAccount> as FromAccountInfo>::try_from_info(#name)?;
                        SystemCpi::create_account(&system_acc, &#payer, &crate::ID, #space as u64, None)?;
                        Mut::try_from_info(#name)?
                    };
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

pub struct Accounts(pub Vec<Account>);

impl Accounts {
    pub fn split_for_impl(&self) -> (NameList, Assign) {
        let (name_list, assign): (Vec<&Ident>, Vec<(&Ident, &PathSegment, &Constraints)>) = self
            .0
            .iter()
            .map(|el| (&el.name, (&el.name, &el.ty, &el.constraints)))
            .unzip();

        (NameList(name_list), Assign(assign))
    }
}
