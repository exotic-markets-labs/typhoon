use typhoon::prelude::*;

#[derive(TyphoonError)]
pub enum TestErrors {
    #[msg("my custom error")]
    Error1,
}
