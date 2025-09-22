use std::{env, fs, path::Path};

fn main() {
    let manifest_dir = env::var("CARGO_WORKSPACE_DIR").unwrap();

    let target_dir = Path::new(&manifest_dir).join("target");
    let idl_dir = target_dir.join("idl");

    fs::create_dir_all(&idl_dir).unwrap();

    let idl = typhoon_idl_generator::generate(&[
        Path::new(&format!("{manifest_dir}programs/escrow")),
        Path::new(&format!("{manifest_dir}interfaces/escrow")),
    ])
    .unwrap();

    fs::write(idl_dir.join("escrow.json"), idl).unwrap();
}
