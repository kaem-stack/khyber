use std::path::PathBuf;

#[derive(Debug, Clone, Default, clap::ValueEnum)]
pub enum Algorithm {
    #[default]
    #[value(name = "ml-kem-768")]
    MlKem768,
}

impl Algorithm {
    pub fn name(&self) -> &'static str {
        match self {
            Algorithm::MlKem768 => "ML-KEM-768",
        }
    }
}

pub struct KeyGenConfig {
    pub algorithm: Algorithm,
    pub out_dir: PathBuf,
}

impl KeyGenConfig {
    pub fn new(out_dir: PathBuf) -> Self {
        Self {
            algorithm: Algorithm::default(),
            out_dir,
        }
    }

    pub fn with_algorithm(mut self, algorithm: Algorithm) -> Self {
        self.algorithm = algorithm;
        self
    }
}
