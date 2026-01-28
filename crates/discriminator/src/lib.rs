#![no_std]

use const_crypto::sha2::Sha256;

pub struct DiscriminatorBuilder<'a> {
    pub name: &'a str,
    pub layout_version: u8,
}

impl<'a> DiscriminatorBuilder<'a> {
    pub const fn new(name: &'a str) -> Self {
        DiscriminatorBuilder {
            name,
            layout_version: 1,
        }
    }

    pub const fn layout(mut self, version: u8) -> Self {
        self.layout_version = version;
        self
    }

    pub const fn build(self) -> [u8; 8] {
        let hasher = Sha256::new().update(self.name.as_bytes());
        let [b0, b1, b2, b3, ..] = hasher.finalize();

        [b0, b1, b2, b3, self.layout_version, 0, 0, 0]
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
