use {
    keys::PrimaryKeys,
    quote::{quote, ToTokens},
    syn::{parse_macro_input, spanned::Spanned, Error, Item, DeriveInput},
    typhoon_discriminator::DiscriminatorBuilder,
};

mod keys;

#[proc_macro_derive(AccountState, attributes(key, no_space))]
pub fn derive_account(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as Item);
    let (attrs, name, generics, fields) = match item {
        Item::Struct(ref item_struct) => (
            &item_struct.attrs,
            &item_struct.ident,
            &item_struct.generics,
            &item_struct.fields,
        ),
        _ => {
            return Error::new(item.span(), "Invalid account type")
                .into_compile_error()
                .into()
        }
    };

    let space_token = if attrs.iter().any(|a| a.path().is_ident("no_space")) {
        None
    } else {
        Some(quote! {
            impl #name {
                pub const SPACE: usize = <#name as Discriminator>::DISCRIMINATOR.len() + core::mem::size_of::<#name>();
            }
        })
    };
    let (_, ty_generics, where_clause) = generics.split_for_impl();

    let keys = match PrimaryKeys::try_from(fields) {
        Ok(fields) => fields,
        Err(err) => return err.to_compile_error().into(),
    };
    let seeded_trait = keys.split_for_impl(name);
    let discriminator = DiscriminatorBuilder::new(&name.to_string()).build();

    quote! {
        impl Owner for #name #ty_generics #where_clause {
            const OWNER: Pubkey = crate::ID;
        }

        impl Discriminator for #name #ty_generics #where_clause {
            const DISCRIMINATOR: &'static [u8] = &[#(#discriminator),*];
        }

        #space_token

        #seeded_trait
    }
    .into_token_stream()
    .into()
}

/// Derive macro for generating optimized batch validation implementations.
/// 
/// This macro generates validation code for account types, including:
/// - Batch validation for multiple accounts of the same type
/// - Pre-validation for early filtering
/// - Single account validation checks
#[proc_macro_derive(FastValidation, attributes(owner, discriminator))]
pub fn fast_validation(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Extract owner pubkey from attributes if specified
    let owner_attr = input.attrs.iter()
        .find(|attr| attr.path().is_ident("owner"))
        .map(|attr| {
            attr.parse_args::<syn::Expr>().unwrap()
        });

    let discriminator = typhoon_discriminator::DiscriminatorBuilder::new(&name.to_string()).build();
    let discriminator_bytes = discriminator.iter().map(|&byte| quote!(#byte));

    let owner_impl = if let Some(owner_expr) = owner_attr {
        quote! {
            impl typhoon_accounts::Owner for #name {
                const OWNER: typhoon_accounts::Pubkey = #owner_expr;
            }
        }
    } else {
        quote! {}
    };

    let expanded = quote! {
        impl typhoon_accounts::Discriminator for #name {
            const DISCRIMINATOR: &'static [u8] = &[#(#discriminator_bytes),*];
        }

        #owner_impl

        impl typhoon_accounts::BatchValidation for #name {
            /// Batch validation with early-exit for multiple accounts.
            #[inline(always)]
            fn validate_batch(accounts: &[&typhoon_accounts::AccountInfo]) -> Result<(), typhoon_accounts::Error> {
                let mut first_error: Option<typhoon_accounts::Error> = None;
                
                for &account in accounts.iter() {
                    // Use pre-validation to quickly filter invalid accounts
                    if !Self::pre_validate(account) {
                        if first_error.is_none() {
                            first_error = Some(typhoon_errors::ErrorCode::AccountOwnedByWrongProgram.into());
                        }
                        continue;
                    }

                    // Perform full validation for accounts that pass pre-validation
                    if let Err(e) = Self::validate_single(account) {
                        if first_error.is_none() {
                            first_error = Some(e);
                        }
                    }
                }

                match first_error {
                    Some(err) => Err(err),
                    None => Ok(()),
                }
            }

            /// Fast pre-validation check using the cheapest possible validation.
            #[inline(always)]
            fn pre_validate(info: &typhoon_accounts::AccountInfo) -> bool {
                // Owner check is the fastest validation we can perform
                info.is_owned_by(&Self::OWNER)
            }
        }

        impl #name {
            /// Single account validation with cheapest check ordering.
            #[inline(always)]
            fn validate_single(info: &typhoon_accounts::AccountInfo) -> Result<(), typhoon_accounts::Error> {
                use typhoon_accounts::*;
                
                // Borrow account data once for all validation checks
                let account_data = info.try_borrow_data()?;
                
                // Check data length first (cheapest and most likely to fail)
                if account_data.len() < Self::DISCRIMINATOR.len() {
                    return Err(pinocchio::program_error::ProgramError::AccountDataTooSmall.into());
                }

                // Validate discriminator
                if Self::DISCRIMINATOR != &account_data[..Self::DISCRIMINATOR.len()] {
                    return Err(typhoon_errors::ErrorCode::AccountDiscriminatorMismatch.into());
                }

                // Verify ownership (redundant with pre_validate, but ensures completeness)
                if !info.is_owned_by(&Self::OWNER) {
                    return Err(typhoon_errors::ErrorCode::AccountOwnedByWrongProgram.into());
                }

                Ok(())
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
