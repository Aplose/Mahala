//! Mahala Blockchain Core
//!
//! Une blockchain légère optimisée pour mobile avec consensus RVS
//! et Dividende Universel conforme à la TRM.

pub mod block;
pub mod chain;
pub mod consensus;
pub mod crypto;
pub mod du;
pub mod storage;
pub mod transaction;
pub mod wallet;
pub mod nft;

pub use block::Block;
pub use chain::Blockchain;
pub use transaction::Transaction;
pub use wallet::WalletAddress;

/// Type pour les hash (32 bytes)
pub type Hash = [u8; 32];

/// Type pour les signatures (64 bytes Ed25519)
pub type Signature = [u8; 64];

/// Type pour les clés publiques (32 bytes Ed25519)
pub type PublicKey = [u8; 32];

/// Type pour les clés privées (32 bytes Ed25519)
pub type PrivateKey = [u8; 32];

/// Timestamp Unix (secondes)
pub type Timestamp = i64;

/// Montant en Mahala (utilise f64 pour la précision décimale)
pub type Amount = f64;
