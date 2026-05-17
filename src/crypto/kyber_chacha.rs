use anyhow::{anyhow, Result};
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce,
};
use ml_kem::{
    kem::{Encapsulate, Key},
    ml_kem_768,
};

use super::algorithm::EncryptionAlgorithm;

// Output wire format: [KEM ciphertext: 1088 bytes] [nonce: 12 bytes] [AEAD ciphertext + tag]
pub struct KyberChaCha;

impl EncryptionAlgorithm for KyberChaCha {
    fn encrypt(&self, public_key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>> {
        let ek_bytes: &Key<ml_kem_768::EncapsulationKey> = public_key
            .try_into()
            .map_err(|_| anyhow!("invalid public key: expected 1184 bytes for ML-KEM-768"))?;
        let ek = ml_kem_768::EncapsulationKey::new(ek_bytes)
            .map_err(|_| anyhow!("malformed public key"))?;

        let (kem_ct, shared_key) = ek.encapsulate();

        let mut nonce_bytes = [0u8; 12];
        getrandom::fill(&mut nonce_bytes).map_err(|e| anyhow!("nonce generation failed: {e}"))?;

        let cipher = ChaCha20Poly1305::new_from_slice(&shared_key)
            .map_err(|_| anyhow!("invalid shared key length"))?;
        let encrypted = cipher
            .encrypt(Nonce::from_slice(&nonce_bytes), plaintext)
            .map_err(|e| anyhow!("symmetric encryption failed: {e}"))?;

        let mut output = kem_ct.to_vec();
        output.extend_from_slice(&nonce_bytes);
        output.extend_from_slice(&encrypted);

        Ok(output)
    }

    fn name(&self) -> &'static str {
        "ML-KEM-768 + ChaCha20-Poly1305"
    }
}
