//! Gestion de la chaîne de blocs Mahala

use std::collections::HashMap;
use crate::{Hash, PublicKey, Amount};
use crate::block::Block;
use crate::transaction::Transaction;
use crate::consensus::rvs::{RVS, RVSConfig};
use crate::du::{DUCalculator, DUConfig};
use crate::storage::checkpoint::{Checkpoint, CheckpointConfig};

/// État de la blockchain (balances, etc.)
#[derive(Debug, Clone)]
pub struct BlockchainState {
    /// Balances des wallets (adresse -> balance)
    pub balances: HashMap<PublicKey, Amount>,
    
    /// Nombre de membres actifs
    pub member_count: u64,
    
    /// Masse monétaire totale
    pub total_mass: Amount,
    
    /// Timestamp de dernière distribution DU
    pub last_du_distribution: i64,
}

impl Default for BlockchainState {
    fn default() -> Self {
        Self {
            balances: HashMap::new(),
            member_count: 0,
            total_mass: 0.0,
            last_du_distribution: chrono::Utc::now().timestamp(),
        }
    }
}

/// Blockchain Mahala
pub struct Blockchain {
    /// Blocs de la chaîne
    blocks: Vec<Block>,
    
    /// État actuel
    state: BlockchainState,
    
    /// Consensus RVS
    consensus: RVS,
    
    /// Calculateur DU
    du_calculator: DUCalculator,
    
    /// Configuration des checkpoints
    checkpoint_config: CheckpointConfig,
    
    /// Checkpoints sauvegardés
    checkpoints: Vec<Checkpoint>,
}

impl Blockchain {
    /// Créer une nouvelle blockchain
    pub fn new(du_config: DUConfig, rvs_config: RVSConfig) -> Self {
        let du_calculator = DUCalculator::new(du_config);
        
        Self {
            blocks: Vec::new(),
            state: BlockchainState::default(),
            consensus: RVS::new(rvs_config),
            du_calculator,
            checkpoint_config: CheckpointConfig::default(),
            checkpoints: Vec::new(),
        }
    }

    /// Créer le bloc genesis
    pub fn create_genesis(&mut self, validator: PublicKey) -> Result<Hash, String> {
        if !self.blocks.is_empty() {
            return Err("Genesis block already exists".to_string());
        }

        let genesis = Block::new(0, [0u8; 32], Vec::new(), validator);
        let mut genesis = genesis;
        let hash = genesis.calculate_hash();
        
        self.blocks.push(genesis);
        
        Ok(hash)
    }

    /// Ajouter un bloc à la chaîne
    pub fn add_block(&mut self, mut block: Block) -> Result<Hash, String> {
        // Vérifier la validité du bloc
        let previous_hash = self.blocks.last()
            .and_then(|b| b.hash)
            .unwrap_or([0u8; 32]);
        
        if !block.is_valid(Some(previous_hash)) {
            return Err("Invalid block".to_string());
        }

        // Vérifier la hauteur
        let expected_height = self.blocks.len() as u64;
        if block.header.height != expected_height {
            return Err(format!("Invalid block height: expected {}, got {}", 
                expected_height, block.header.height));
        }

        // Vérifier le quorum
        let total_validators = self.consensus.active_validators_count();
        if total_validators > 0 && !block.has_quorum(total_validators) {
            return Err("Block does not have required quorum".to_string());
        }

        // Calculer le hash
        let hash = block.calculate_hash();
        
        // Appliquer les transactions
        self.apply_transactions(&block.transactions)?;
        
        // Distribuer le DU si nécessaire
        self.distribute_du_if_needed()?;
        
        // Ajouter le bloc
        self.blocks.push(block);
        
        // Créer un checkpoint si nécessaire
        if self.checkpoint_config.is_checkpoint(expected_height) {
            self.create_checkpoint();
        }
        
        Ok(hash)
    }

    /// Appliquer les transactions d'un bloc
    fn apply_transactions(&mut self, transactions: &[Transaction]) -> Result<(), String> {
        for tx in transactions {
            // Vérifier que l'expéditeur a suffisamment de fonds
            let sender_balance = self.state.balances.get(&tx.from)
                .copied()
                .unwrap_or(0.0);
            
            let total_needed = tx.amount + tx.fee;
            if sender_balance < total_needed {
                return Err(format!("Insufficient balance: need {}, have {}", 
                    total_needed, sender_balance));
            }

            // Débiter l'expéditeur
            *self.state.balances.entry(tx.from).or_insert(0.0) -= total_needed;
            
            // Créditer le destinataire
            *self.state.balances.entry(tx.to).or_insert(0.0) += tx.amount;
            
            // Les frais sont brûlés (réduisent la masse monétaire)
            self.state.total_mass -= tx.fee;
        }
        
        Ok(())
    }

    /// Distribuer le Dividende Universel si nécessaire
    fn distribute_du_if_needed(&mut self) -> Result<(), String> {
        let now = chrono::Utc::now().timestamp();
        
        // Distribuer le DU quotidiennement
        // (Dans une vraie implémentation, on vérifierait si un jour s'est écoulé)
        let days_since = (now - self.state.last_du_distribution) / 86400;
        
        if days_since >= 1 {
            let du = self.du_calculator.calculate_current_du(
                self.state.total_mass,
                self.state.member_count,
            );
            
            // Distribuer le DU à tous les membres
            if self.state.member_count > 0 && du > 0.0 {
                let du_per_member = du;
                let total_du = du_per_member * (self.state.member_count as f64);
                
                // Créditer chaque membre
                for (_, balance) in self.state.balances.iter_mut() {
                    *balance += du_per_member;
                }
                
                // Mettre à jour la masse monétaire
                self.state.total_mass += total_du;
            }
            
            self.state.last_du_distribution = now;
        }
        
        Ok(())
    }

    /// Créer un checkpoint
    fn create_checkpoint(&mut self) {
        if let Some(block) = self.blocks.last() {
            let state_hash = self.calculate_state_hash();
            let checkpoint = Checkpoint::from_block(
                block,
                state_hash,
                self.checkpoint_config.interval,
            );
            self.checkpoints.push(checkpoint);
        }
    }

    /// Calculer le hash de l'état
    fn calculate_state_hash(&self) -> Hash {
        use crate::crypto::hash_data;
        use serde_json;
        
        let state_data = serde_json::to_vec(&self.state.balances)
            .expect("Serialization should never fail");
        hash_data(&state_data)
    }

    /// Obtenir la hauteur actuelle
    pub fn height(&self) -> u64 {
        self.blocks.len() as u64
    }

    /// Obtenir le hash du dernier bloc
    pub fn last_block_hash(&self) -> Option<Hash> {
        self.blocks.last().and_then(|b| b.hash)
    }

    /// Obtenir la balance d'un wallet
    pub fn get_balance(&self, address: &PublicKey) -> Amount {
        self.state.balances.get(address).copied().unwrap_or(0.0)
    }

    /// Obtenir le consensus
    pub fn consensus_mut(&mut self) -> &mut RVS {
        &mut self.consensus
    }

    /// Obtenir le consensus (immutable)
    pub fn consensus(&self) -> &RVS {
        &self.consensus
    }

    /// Obtenir un bloc par hauteur
    pub fn get_block(&self, height: u64) -> Option<&Block> {
        self.blocks.get(height as usize)
    }

    /// Obtenir le dernier checkpoint
    pub fn last_checkpoint(&self) -> Option<&Checkpoint> {
        self.checkpoints.last()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::keys::KeyPair;

    #[test]
    fn test_blockchain_creation() {
        let blockchain = Blockchain::new(DUConfig::default(), RVSConfig::default());
        assert_eq!(blockchain.height(), 0);
    }

    #[test]
    fn test_genesis_block() {
        let mut blockchain = Blockchain::new(DUConfig::default(), RVSConfig::default());
        let keypair = KeyPair::new();
        
        let result = blockchain.create_genesis(*keypair.public_key());
        assert!(result.is_ok());
        assert_eq!(blockchain.height(), 1);
    }

    #[test]
    fn test_add_block() {
        let mut blockchain = Blockchain::new(DUConfig::default(), RVSConfig::default());
        let keypair = KeyPair::new();
        
        blockchain.create_genesis(*keypair.public_key()).unwrap();
        
        let previous_hash = blockchain.last_block_hash().unwrap();
        let mut block = Block::new(1, previous_hash, Vec::new(), *keypair.public_key());
        block.calculate_hash(); // Calculer le hash avant d'ajouter
        
        let result = blockchain.add_block(block);
        assert!(result.is_ok());
        assert_eq!(blockchain.height(), 2);
    }
}

