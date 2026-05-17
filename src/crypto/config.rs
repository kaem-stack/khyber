use crate::keys::Algorithm;

pub struct EncryptConfig {
    pub algorithm: Algorithm,
}

impl Default for EncryptConfig {
    fn default() -> Self {
        Self {
            algorithm: Algorithm::default(),
        }
    }
}

impl EncryptConfig {
    pub fn with_algorithm(mut self, algorithm: Algorithm) -> Self {
        self.algorithm = algorithm;
        self
    }
}
