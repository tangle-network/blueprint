use super::bn254::Bn254Backend;
use super::ecdsa::EcdsaBackend;

pub trait EigenlayerBackend: Bn254Backend + EcdsaBackend {}

impl<T> EigenlayerBackend for T where T: Bn254Backend + EcdsaBackend {}
