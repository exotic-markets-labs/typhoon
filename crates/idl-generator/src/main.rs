use {codama::Codama, std::path::Path, typhoon_idl_generator::plugin::TyphoonPlugin};

pub fn main() {
    // let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let p = Path::new("/home/aursen/crayfish/examples/instruction-data");
    let codama = Codama::load(p)
        .unwrap()
        .without_default_plugin()
        .add_plugin(TyphoonPlugin);
    println!("{}", codama.get_json_idl().unwrap());
}
