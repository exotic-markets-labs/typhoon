use typhoon::prelude::*;

#[derive(TyphoonError)]
pub enum SeedsError {
    #[msg("Error: Invalid owner")]
    InvalidOwner = 200,
}
