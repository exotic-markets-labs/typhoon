#[macro_export]
macro_rules! error {
    ($account_name: expr, $err: literal) => {
        return Err(syn::Error::new_spanned($account_name, $err));
    };
}
