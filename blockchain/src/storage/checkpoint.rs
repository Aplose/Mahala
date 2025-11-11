//! Points de contrôle (checkpoints) pour synchronisation légère
//!
//! Permet aux clients légers de se synchroniser rapidement en téléchargeant
//! uniquement les en-têtes de blocs et les checkpoints périodiques

use serde::{Deserialize, Serialize};
use crate::{Hash, Timestamp};
use crate::block::Block;

/// Point de contrôle (checkpoint)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    /// Hauteur du bloc checkpoint
    pub height: u64,
    
    /// Hash du bloc
    pub block_hash: Hash,
    
    /// Hash de l'état (balances, etc.)
    pub state_hash: Hash,
    
    /// Timestamp de création
    pub timestamp: Timestamp,
    
    /// Période entre checkpoints (en blocs)
    pub checkpoint_interval: u64,
}

impl Checkpoint {
    /// Créer un nouveau checkpoint depuis un bloc
    pub fn from_block(block: &Block, state_hash: Hash, interval: u64) -> Self {
        Self {
            height: block.header.height,
            block_hash: block.hash.unwrap_or_else(|| {
                let mut b = block.clone();
                b.calculate_hash();
                b.hash.unwrap()
            }),
            state_hash,
            timestamp: block.header.timestamp,
            checkpoint_interval: interval,
        }
    }

    /// Vérifier si un checkpoint est valide
    pub fn is_valid(&self) -> bool {
        self.height > 0 && self.checkpoint_interval > 0
    }

    /// Obtenir le prochain checkpoint attendu
    pub fn next_checkpoint_height(&self) -> u64 {
        self.height + self.checkpoint_interval
    }
}

/// Configuration des checkpoints
#[derive(Debug, Clone)]
pub struct CheckpointConfig {
    /// Intervalle entre checkpoints (en blocs)
    pub interval: u64,
    
    /// Hauteur du premier checkpoint
    pub first_checkpoint: u64,
}

impl Default for CheckpointConfig {
    fn default() -> Self {
        Self {
            interval: 100, // Checkpoint tous les 100 blocs
            first_checkpoint: 100,
        }
    }
}

impl CheckpointConfig {
    /// Vérifier si un bloc doit être un checkpoint
    pub fn is_checkpoint(&self, height: u64) -> bool {
        height >= self.first_checkpoint && (height % self.interval == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::keys::KeyPair;

    #[test]
    fn test_checkpoint_creation() {
        let keypair = KeyPair::new();
        let block = crate::block::Block::new(
            100,
            [0u8; 32],
            Vec::new(),
            *keypair.public_key(),
        );
        
        let state_hash = [1u8; 32];
        let checkpoint = Checkpoint::from_block(&block, state_hash, 100);
        
        assert_eq!(checkpoint.height, 100);
        assert!(checkpoint.is_valid());
    }

    #[test]
    fn test_checkpoint_config() {
        let config = CheckpointConfig::default();
        
        assert!(config.is_checkpoint(100));
        assert!(config.is_checkpoint(200));
        assert!(!config.is_checkpoint(150));
        assert!(!config.is_checkpoint(50));
    }
}

