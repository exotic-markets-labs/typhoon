use {codama::Codama, std::path::Path, typhoon_metadata_extractor::plugin::TyphoonPlugin};

pub fn main() {
    // let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let p = Path::new("/home/aursen/crayfish/examples/counter");
    let codama = Codama::load(p)
        .unwrap()
        .without_default_plugin()
        .add_plugin(TyphoonPlugin);
    println!("{}", codama.get_json_idl().unwrap());
}
