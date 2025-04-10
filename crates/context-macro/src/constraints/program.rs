use syn::{
    parse::{Parse, ParseStream},
    Expr, Token,
};

#[derive(Clone)]
pub struct ConstraintProgram {
    _program_id: Expr,
}

impl Parse for ConstraintProgram {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![=]>()?;
        let program_id = input.parse()?;

        Ok(ConstraintProgram {
            _program_id: program_id,
        })
    }
}
