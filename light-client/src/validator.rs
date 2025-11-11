//! Participation au consensus RVS
//!
//! Permet au light client de participer à la validation des blocs
//! quand il est sélectionné par le consensus

use mahala_blockchain::{PublicKey, Hash, Signature};
use mahala_blockchain::block::Block;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Participant au consensus
pub struct ValidatorParticipant {
    /// Clé publique du validateur
    pub public_key: Option<PublicKey>,
    
    /// Statistiques de participation
    pub stats: ValidatorStats,
}

/// Statistiques du validateur
#[derive(Debug, Clone, Default)]
pub struct ValidatorStats {
    /// Nombre de fois sélectionné
    pub times_selected: u64,
    
    /// Nombre de blocs signés
    pub blocks_signed: u64,
    
    /// Nombre de fois absent (sélectionné mais pas signé)
    pub absences: u64,
}

impl ValidatorParticipant {
    /// Créer un nouveau participant
    pub fn new() -> Self {
        Self {
            public_key: None,
            stats: ValidatorStats::default(),
        }
    }

    /// Enregistrer le validateur
    pub fn register(&mut self, public_key: PublicKey) {
        self.public_key = Some(public_key);
    }

    /// Signer un bloc si sélectionné
    pub async fn sign_block_if_selected(
        &mut self,
        block: &mut Block,
        private_key: &mahala_blockchain::PrivateKey,
    ) -> Result<bool, String> {
        let Some(public_key) = self.public_key else {
            return Ok(false);
        };

        // Vérifier si ce validateur est dans la sélection
        // (Cette vérification devrait être faite avant d'appeler cette fonction)
        
        // Calculer le hash du bloc
        let hash = block.calculate_hash();
        
        // Signer le hash
        use mahala_blockchain::crypto::sign;
        let signature = sign(&hash, private_key);
        
        // Ajouter la signature au bloc
        block.add_validator_signature(public_key, signature);
        
        self.stats.blocks_signed += 1;
        Ok(true)
    }

    /// Mettre à jour les statistiques
    pub fn update_stats(&mut self, selected: bool, signed: bool) {
        if selected {
            self.stats.times_selected += 1;
            if !signed {
                self.stats.absences += 1;
            }
        }
    }
}

impl Default for ValidatorParticipant {
    fn default() -> Self {
        Self::new()
    }
}

