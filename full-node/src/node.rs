//! Nœud complet Mahala

use mahala_blockchain::{Blockchain, Block, PublicKey};
use mahala_blockchain::du::DUConfig;
use mahala_blockchain::consensus::rvs::RVSConfig;
use crate::mempool::Mempool;
use crate::config::NodeConfig;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};

/// Nœud complet de la blockchain Mahala
pub struct FullNode {
    /// Blockchain
    blockchain: Arc<RwLock<Blockchain>>,
    
    /// Mempool
    mempool: Arc<Mempool>,
    
    /// Configuration
    config: NodeConfig,
    
    /// Clé publique du nœud (validateur)
    validator_key: PublicKey,
}

impl FullNode {
    /// Créer un nouveau nœud complet
    pub fn new(config: NodeConfig, validator_key: PublicKey) -> Self {
        let du_config = DUConfig::default();
        let rvs_config = RVSConfig::default();
        let blockchain = Arc::new(RwLock::new(Blockchain::new(du_config, rvs_config)));
        
        let mempool = Arc::new(Mempool::new(10000, 3600)); // 10k tx max, 1h max age
        
        Self {
            blockchain,
            mempool,
            config,
            validator_key,
        }
    }

    /// Démarrer le nœud
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Créer le bloc genesis si nécessaire
        {
            let mut blockchain = self.blockchain.write().await;
            if blockchain.height() == 0 {
                blockchain.create_genesis(self.validator_key)?;
            }
        }

        // Démarrer la production de blocs
        let blockchain_clone = self.blockchain.clone();
        let mempool_clone = self.mempool.clone();
        let validator_key = self.validator_key;
        
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(5)); // Bloc toutes les 5 secondes
            
            loop {
                interval.tick().await;
                
                // Produire un nouveau bloc
                if let Err(e) = Self::produce_block(
                    &blockchain_clone,
                    &mempool_clone,
                    validator_key,
                ).await {
                    eprintln!("Erreur production bloc: {}", e);
                }
            }
        });

        // Nettoyer le mempool périodiquement
        let mempool_clone = self.mempool.clone();
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(60)); // Toutes les minutes
            loop {
                interval.tick().await;
                mempool_clone.cleanup_expired().await;
            }
        });

        Ok(())
    }

    /// Produire un nouveau bloc
    async fn produce_block(
        blockchain: &Arc<RwLock<Blockchain>>,
        mempool: &Arc<Mempool>,
        validator_key: PublicKey,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let height;
        let previous_hash;
        
        {
            let blockchain_guard = blockchain.read().await;
            height = blockchain_guard.height();
            previous_hash = blockchain_guard.last_block_hash()
                .ok_or("No previous block")?;
        }

        // Obtenir les transactions du mempool
        let transactions = mempool.get_transactions_for_block(100).await;

        // Créer le bloc
        let mut block = Block::new(height, previous_hash, transactions.clone(), validator_key);
        block.calculate_hash();

        // Ajouter le bloc à la chaîne
        {
            let mut blockchain_guard = blockchain.write().await;
            blockchain_guard.add_block(block)?;
        }

        // Retirer les transactions du mempool
        let hashes: Vec<_> = transactions
            .iter()
            .filter_map(|tx| tx.hash)
            .collect();
        mempool.remove_transactions(&hashes).await;

        Ok(())
    }

    /// Obtenir la blockchain (pour l'API)
    pub fn blockchain(&self) -> Arc<RwLock<Blockchain>> {
        self.blockchain.clone()
    }

    /// Obtenir le mempool (pour l'API)
    pub fn mempool(&self) -> Arc<Mempool> {
        self.mempool.clone()
    }

    /// Ajouter une transaction au mempool
    pub async fn add_transaction(&self, tx: mahala_blockchain::transaction::Transaction) -> Result<(), crate::mempool::MempoolError> {
        self.mempool.add_transaction(tx).await
    }
}

