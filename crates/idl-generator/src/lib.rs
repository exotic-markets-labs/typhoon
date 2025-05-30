mod doc;
mod helpers;
pub mod plugin;
pub mod visitors;

use {
    crate::plugin::TyphoonPlugin,
    codama::{Codama, CodamaResult},
    std::path::Path,
};

pub fn generate(manifest_path: impl AsRef<Path>) -> CodamaResult<String> {
    let codama = Codama::load(manifest_path)
        .unwrap()
        .without_default_plugin()
        .add_plugin(TyphoonPlugin);

    codama.get_json_idl()
}
