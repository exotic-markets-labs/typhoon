use {
    crate::constraints::Constraints,
    proc_macro2::{Span, TokenStream},
    quote::{quote, ToTokens},
    std::ops::Deref,
    syn::{
        spanned::Spanned, visit_mut::VisitMut, Field, GenericArgument, Ident, PathArguments,
        PathSegment, Type, TypePath,
    },
};

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
                            let signer_seeds = [#punctuated_seeds, &[bumps.#name as u8]];
                            // TODO: make it work when not using pinocchio
                            let seeds_vec = &signer_seeds.into_iter().map(|seed| typhoon_program::SignerSeed::from(seed)).collect::<Vec<typhoon_program::SignerSeed>>()[..];
                            let signer: typhoon_program::SignerSeeds = typhoon_program::SignerSeeds::from(&seeds_vec[..]);
                            typhoon::lib::SystemCpi::create_account(&system_acc, &#payer, &crate::ID, #space as u64, Some(&[typhoon_program::SignerSeeds::from(signer)]))?;
                            Mut::try_from_info(#name)?
                        };
                    }
                } else if c.is_seeded() {
                    let Some(keys) = c.get_keys() else {
                        return syn::Error::new(name.span(), "Seeded accounts require `keys` to be passed on init").to_compile_error()
                    };

                    fn get_subsegments(segment: &PathSegment) -> Vec<PathSegment> {
                        if let PathArguments::AngleBracketed(arguments) = &segment.arguments {
                            arguments.args.iter().filter_map(|a| match a {
                                GenericArgument::Type(Type::Path(p)) => Some(p.path.segments.clone()),
                                _ => None,
                            }).flatten().collect()
                        } else {
                            vec![]
                        }
                    }

                    let mut segment = (*ty).clone();
                    let mut subsegments = get_subsegments(ty);
                    let error = syn::Error::new(ty.span(), "Unexpected type structure").to_compile_error();
                    while segment.ident != "Account" {
                        let Some(s) = subsegments.first() else {
                            return error
                        };

                        segment = s.clone().clone();
                        subsegments = get_subsegments(&segment);
                    }
                    let Some(s) = subsegments.first() else {
                        return error
                    };
                    let account_ty = &s.ident;


                    quote! {
                        let #name: #ty = {
                            let system_acc = <typhoon::lib::Mut<typhoon::lib::SystemAccount> as typhoon::lib::FromAccountInfo>::try_from_info(#name)?;
                            // TODO: avoid reusing seeds here and in verifications
                            let bump = [bumps.#name as u8];
                            let signer_seeds = #account_ty::derive_with_bump(#keys, &bump);
                            // TODO: make it work when not using pinocchio
                            let seeds_vec = &signer_seeds.into_iter().map(|seed| typhoon_program::SignerSeed::from(seed)).collect::<Vec<typhoon_program::SignerSeed>>()[..];
                            let signer: typhoon_program::SignerSeeds = typhoon_program::SignerSeeds::from(&seeds_vec[..]);
                            typhoon::lib::SystemCpi::create_account(&system_acc, &#payer, &crate::ID, #space as u64, Some(&[typhoon_program::SignerSeeds::from(signer)]))?;
                            Mut::try_from_info(#name)?
                        };
                    }
                } else {
                    quote! {
                        let #name: #ty = {
                            let system_acc = <typhoon::lib::Mut<typhoon::lib::SystemAccount> as typhoon::lib::FromAccountInfo>::try_from_info(#name)?;
                            typhoon::lib::SystemCpi::create_account(&system_acc, &#payer, &crate::ID, #space as u64, None)?;
                            Mut::try_from_info(#name)?
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

pub struct Accounts(pub Vec<Account>);

impl Accounts {
    pub fn split_for_impl(&self) -> (NameList, Assign) {
        let (names, assigns) = self
            .0
            .iter()
            .map(|el| (&el.name, (&el.name, &el.ty, &el.constraints)))
            .unzip();

        (NameList(names), Assign(assigns))
    }
}
