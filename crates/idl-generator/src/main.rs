use typhoon_idl_generator::generate;

pub fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("{}", generate(manifest_dir).unwrap());
}
