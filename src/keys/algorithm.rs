use anyhow::Result;
use kaem_sdk::modules::khyber::generate_keys::KeysGeneratedEvent;

pub trait KemAlgorithm {
    fn generate(&self) -> Result<KeysGeneratedEvent>;
}
