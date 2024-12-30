use syn::{
    parse::{Parse, ParseStream},
    Expr, Token,
};

#[derive(Clone)]
pub struct ConstraintBump {
    pub bump: Option<Expr>,
    pub find_canonical: bool,
}

impl ConstraintBump {
    pub fn is_some(&self) -> bool {
        self.bump.is_some() || self.find_canonical
    }
}

impl Parse for ConstraintBump {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![,]) {
            Ok(ConstraintBump {
                bump: None,
                find_canonical: true,
            })
        } else {
            input.parse::<Token![=]>()?;
            let bump = input.parse()?;

            Ok(ConstraintBump {
                bump: Some(bump),
                find_canonical: false,
            })
        }
    }
}
