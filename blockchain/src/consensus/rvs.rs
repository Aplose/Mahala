//! Consensus RVS (Random Validator Selection)
//!
//! Sélection aléatoire de validateurs toutes les 5 secondes
//! avec quorum de 67% pour valider les blocs

use crate::{PublicKey, Hash, Timestamp};
use crate::consensus::vrf::VRF;
use std::collections::HashMap;

/// Configuration du consensus RVS
#[derive(Debug, Clone)]
pub struct RVSConfig {
    /// Intervalle entre les blocs (en secondes)
    pub block_interval: u64,
    
    /// Nombre de validateurs à sélectionner
    pub validator_count: usize,
    
    /// Quorum requis (en pourcentage, 67 par défaut)
    pub quorum_percentage: u8,
}

impl Default for RVSConfig {
    fn default() -> Self {
        Self {
            block_interval: 5, // 5 secondes
            validator_count: 10, // 10 validateurs par bloc
            quorum_percentage: 67,
        }
    }
}

/// Gestionnaire du consensus RVS
pub struct RVS {
    config: RVSConfig,
    /// Registre des validateurs actifs (wallet -> clé publique)
    validators: HashMap<PublicKey, ValidatorInfo>,
}

/// Informations sur un validateur
#[derive(Debug, Clone)]
pub struct ValidatorInfo {
    /// Clé publique du validateur
    pub public_key: PublicKey,
    
    /// Adresse du wallet
    pub wallet_address: PublicKey,
    
    /// Score de réputation (basé sur l'historique)
    pub reputation: f64,
    
    /// Timestamp de dernière activité
    pub last_active: Timestamp,
}

/// Résultat de la sélection de validateurs
#[derive(Debug, Clone)]
pub struct ValidatorSelection {
    /// Validateurs sélectionnés pour ce bloc
    pub selected_validators: Vec<PublicKey>,
    
    /// Hash du bloc précédent utilisé pour la sélection
    pub previous_block_hash: Hash,
    
    /// Timestamp de la sélection
    pub timestamp: Timestamp,
}

impl RVS {
    /// Créer un nouveau gestionnaire RVS
    pub fn new(config: RVSConfig) -> Self {
        Self {
            config,
            validators: HashMap::new(),
        }
    }

    /// Enregistrer un validateur
    pub fn register_validator(&mut self, public_key: PublicKey, wallet_address: PublicKey) {
        self.validators.insert(public_key, ValidatorInfo {
            public_key,
            wallet_address,
            reputation: 1.0,
            last_active: chrono::Utc::now().timestamp(),
        });
    }

    /// Désenregistrer un validateur
    pub fn unregister_validator(&mut self, public_key: &PublicKey) {
        self.validators.remove(public_key);
    }

    /// Sélectionner les validateurs pour le prochain bloc
    pub fn select_validators(&self, previous_block_hash: Hash) -> ValidatorSelection {
        let mut candidates: Vec<(PublicKey, u64)> = Vec::new();
        
        // Utiliser le hash du bloc précédent comme seed pour la sélection
        let seed = &previous_block_hash;
        
        // Calculer un score VRF pour chaque validateur
        for (pub_key, info) in &self.validators {
            // Créer un message unique pour ce validateur
            let mut message = Vec::with_capacity(32 + 32);
            message.extend_from_slice(seed);
            message.extend_from_slice(pub_key);
            
            // Générer une sortie VRF (simulée avec hash)
            // Dans une vraie implémentation, chaque validateur générerait sa propre VRF
            let vrf_output = crate::crypto::hash_data(&message);
            let score = VRF::output_to_number(&vrf_output);
            
            // Ajuster le score avec la réputation
            let adjusted_score = (score as f64 * info.reputation) as u64;
            
            candidates.push((*pub_key, adjusted_score));
        }
        
        // Trier par score (décroissant)
        candidates.sort_by(|a, b| b.1.cmp(&a.1));
        
        // Sélectionner les N meilleurs
        let selected: Vec<PublicKey> = candidates
            .iter()
            .take(self.config.validator_count)
            .map(|(key, _)| *key)
            .collect();
        
        ValidatorSelection {
            selected_validators: selected,
            previous_block_hash,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    /// Vérifier si un validateur est sélectionné
    pub fn is_validator_selected(&self, public_key: &PublicKey, selection: &ValidatorSelection) -> bool {
        selection.selected_validators.contains(public_key)
    }

    /// Calculer le quorum requis
    pub fn required_quorum(&self) -> usize {
        let total = self.validators.len();
        (total * self.config.quorum_percentage as usize + 99) / 100
    }

    /// Mettre à jour la réputation d'un validateur
    pub fn update_reputation(&mut self, public_key: &PublicKey, success: bool) {
        if let Some(info) = self.validators.get_mut(public_key) {
            if success {
                // Augmenter légèrement la réputation
                info.reputation = (info.reputation * 1.01).min(2.0);
            } else {
                // Diminuer la réputation
                info.reputation = (info.reputation * 0.95).max(0.1);
            }
            info.last_active = chrono::Utc::now().timestamp();
        }
    }

    /// Obtenir le nombre de validateurs actifs
    pub fn active_validators_count(&self) -> usize {
        self.validators.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::keys::KeyPair;

    #[test]
    fn test_rvs_creation() {
        let rvs = RVS::new(RVSConfig::default());
        assert_eq!(rvs.active_validators_count(), 0);
    }

    #[test]
    fn test_register_validator() {
        let mut rvs = RVS::new(RVSConfig::default());
        let keypair = KeyPair::new();
        
        rvs.register_validator(*keypair.public_key(), *keypair.public_key());
        assert_eq!(rvs.active_validators_count(), 1);
    }

    #[test]
    fn test_select_validators() {
        let mut rvs = RVS::new(RVSConfig {
            validator_count: 3,
            ..Default::default()
        });
        
        // Enregistrer 5 validateurs
        for _ in 0..5 {
            let keypair = KeyPair::new();
            rvs.register_validator(*keypair.public_key(), *keypair.public_key());
        }
        
        let previous_hash = crate::crypto::hash_data(b"test");
        let selection = rvs.select_validators(previous_hash);
        
        assert_eq!(selection.selected_validators.len(), 3);
    }

    #[test]
    fn test_quorum_calculation() {
        let mut rvs = RVS::new(RVSConfig::default());
        
        // 10 validateurs, quorum 67% = 7
        for _ in 0..10 {
            let keypair = KeyPair::new();
            rvs.register_validator(*keypair.public_key(), *keypair.public_key());
        }
        
        assert_eq!(rvs.required_quorum(), 7);
    }
}

