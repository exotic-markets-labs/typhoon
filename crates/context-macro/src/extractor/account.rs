use {
    proc_macro2::TokenStream,
    quote::ToTokens,
    syn::{GenericArgument, PathArguments, PathSegment, Type},
};

pub struct AccountExtractor<'a>(pub &'a PathSegment);

impl AccountExtractor<'_> {
    pub fn get_account_type(&self) -> TokenStream {
        let mut segment = (*self.0).clone();
        let mut subsegments = get_subsegments(&segment);
        let error =
            syn::Error::new(segment.ident.span(), "Unexpected type structure").to_compile_error();
        while segment.ident != "Account" {
            let Some(s) = subsegments.first() else {
                return error;
            };

            segment = s.clone().clone();
            subsegments = get_subsegments(&segment);
        }
        let Some(s) = subsegments.first() else {
            return error;
        };

        s.ident.to_token_stream()
    }
}

fn get_subsegments(segment: &PathSegment) -> Vec<PathSegment> {
    if let PathArguments::AngleBracketed(arguments) = &segment.arguments {
        arguments
            .args
            .iter()
            .filter_map(|a| match a {
                GenericArgument::Type(Type::Path(p)) => Some(p.path.segments.clone()),
                _ => None,
            })
            .flatten()
            .collect()
    } else {
        vec![]
    }
}
