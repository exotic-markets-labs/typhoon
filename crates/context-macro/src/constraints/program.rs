use syn::{
    parse::{Parse, ParseStream},
    Expr, Token,
};

#[derive(Clone)]
pub struct ConstraintProgram(pub Expr);

impl Parse for ConstraintProgram {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![=]>()?;
        let program_id = input.parse()?;

        Ok(ConstraintProgram(program_id))
    }
}
