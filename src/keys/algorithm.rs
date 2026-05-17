use anyhow::Result;

pub struct KeyPair {
    pub public: Vec<u8>,
    pub secret: Vec<u8>,
}

pub trait KemAlgorithm {
    fn generate(&self) -> Result<KeyPair>;
    fn name(&self) -> &'static str;
}
