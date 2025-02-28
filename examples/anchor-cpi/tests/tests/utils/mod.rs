use solana_nostd_sha256::hashv;

pub const SIGHASH_GLOBAL_NAMESPACE: &str = "global";

pub fn sighash(namespace: &str, name: &str) -> [u8; 8] {
    let preimage = format!("{namespace}:{name}");
    let hash = hashv(&[preimage.as_bytes()]);

    hash[..8].try_into().unwrap()
}
