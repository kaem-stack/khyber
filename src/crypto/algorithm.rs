use anyhow::Result;

pub trait EncryptionAlgorithm {
    fn encrypt(&self, public_key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>>;
    fn decrypt(&self, secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>>;
    fn name(&self) -> &'static str;
}
