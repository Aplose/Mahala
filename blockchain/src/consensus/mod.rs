//! Consensus RVS (Random Validator Selection) pour Mahala
//!
//! Sélection aléatoire de validateurs avec VRF (Verifiable Random Function)
//! pour garantir la décentralisation et la sécurité

pub mod rvs;
pub mod vrf;

pub use rvs::RVS;
pub use vrf::VRF;

