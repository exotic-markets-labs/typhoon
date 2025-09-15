#[derive(Default)]
pub struct AccountMeta {
    pub is_signer: bool,
    pub is_mutable: bool,
    pub is_optional: bool,
}
