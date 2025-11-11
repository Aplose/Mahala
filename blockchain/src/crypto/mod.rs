//! Module cryptographique pour Mahala
//!
//! Utilise Ed25519 pour les signatures et Blake3 pour le hashing
//! pour une performance optimale sur mobile.

pub mod keys;
pub mod signatures;
pub mod hash;

pub use keys::{KeyPair, generate_keypair};
pub use signatures::{sign, verify_signature};
pub use hash::{hash_data, hash_to_string};


