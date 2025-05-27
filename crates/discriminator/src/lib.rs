#![no_std]

use sha2::{Digest, Sha256};

pub struct DiscriminatorBuilder<'a> {
    pub name: &'a str,
    pub layout_version: u8,
}

impl<'a> DiscriminatorBuilder<'a> {
    pub fn new(name: &'a str) -> Self {
        DiscriminatorBuilder {
            name,
            layout_version: 1,
        }
    }

    pub fn layout(mut self, version: u8) -> Self {
        self.layout_version = version;
        self
    }

    pub fn build(self) -> [u8; 8] {
        let mut hasher = Sha256::new();
        hasher.update(self.name);
        let hash = hasher.finalize();

        let mut discriminator = [0; 8];
        discriminator[..4].copy_from_slice(&hash[..4]);
        discriminator[4] = self.layout_version;

        discriminator
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discriminator_test() {
        let discriminator = DiscriminatorBuilder::new("state").build();
        let expected = [75, 166, 151, 53, 1, 0, 0, 0];

        assert_eq!(discriminator, expected);

        let discriminator = DiscriminatorBuilder::new("state").layout(2).build();
        let expected = [75, 166, 151, 53, 2, 0, 0, 0];

        assert_eq!(discriminator, expected);
    }
}
