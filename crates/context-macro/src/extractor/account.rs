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
    match &segment.arguments {
        PathArguments::AngleBracketed(arguments) => {
            let mut segments = Vec::with_capacity(arguments.args.len());
            for arg in arguments.args.iter() {
                if let GenericArgument::Type(Type::Path(p)) = arg {
                    segments.extend(p.path.segments.iter().cloned());
                }
            }
            segments
        }
        _ => Vec::new(),
    }
}
