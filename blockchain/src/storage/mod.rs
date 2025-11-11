//! Stockage léger pour la blockchain Mahala
//!
//! Optimisé pour mobile avec checkpoints et arbres de Merkle

pub mod merkle;
pub mod checkpoint;

pub use merkle::MerkleTree;
pub use checkpoint::Checkpoint;

