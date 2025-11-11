//! Stockage local pour le light client
//!
//! Stocke uniquement les données essentielles pour minimiser
//! l'utilisation de l'espace disque sur mobile

use mahala_blockchain::{Hash, PublicKey, Amount};
use mahala_blockchain::storage::checkpoint::Checkpoint;
use serde::{Deserialize, Serialize};

/// État local du light client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightClientState {
    /// Adresse du wallet
    pub wallet_address: PublicKey,
    
    /// Balance actuelle
    pub balance: Amount,
    
    /// Dernier checkpoint synchronisé
    pub last_checkpoint: Option<Checkpoint>,
    
    /// Hauteur synchronisée
    pub synced_height: u64,
    
    /// Hash du dernier bloc connu
    pub last_block_hash: Option<Hash>,
}

impl LightClientState {
    /// Créer un nouvel état
    pub fn new(wallet_address: PublicKey) -> Self {
        Self {
            wallet_address,
            balance: 0.0,
            last_checkpoint: None,
            synced_height: 0,
            last_block_hash: None,
        }
    }

    /// Sauvegarder l'état (à implémenter avec un système de stockage)
    pub fn save(&self) -> Result<(), String> {
        // TODO: Implémenter la sauvegarde (SQLite, JSON, etc.)
        Ok(())
    }

    /// Charger l'état
    pub fn load(wallet_address: &PublicKey) -> Result<Self, String> {
        // TODO: Implémenter le chargement
        Ok(Self::new(*wallet_address))
    }
}

