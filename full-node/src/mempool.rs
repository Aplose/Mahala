//! Mempool (pool de transactions en attente)

use mahala_blockchain::transaction::Transaction;
use mahala_blockchain::Hash;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;

/// Transaction dans le mempool avec métadonnées
#[derive(Debug, Clone)]
struct MempoolEntry {
    transaction: Transaction,
    received_at: u64,
    priority: f64,
}

/// Mempool pour stocker les transactions en attente
pub struct Mempool {
    /// Transactions indexées par hash
    transactions: RwLock<HashMap<Hash, MempoolEntry>>,
    
    /// Taille maximale du mempool
    max_size: usize,
    
    /// Durée de vie maximale d'une transaction (secondes)
    max_age: u64,
}

impl Mempool {
    /// Créer un nouveau mempool
    pub fn new(max_size: usize, max_age: u64) -> Self {
        Self {
            transactions: RwLock::new(HashMap::new()),
            max_size,
            max_age,
        }
    }

    /// Ajouter une transaction au mempool
    pub async fn add_transaction(&self, tx: Transaction) -> Result<(), MempoolError> {
        // Vérifier la validité
        if !tx.is_valid() {
            return Err(MempoolError::InvalidTransaction);
        }

        // Calculer le hash
        let mut tx_copy = tx.clone();
        let hash = tx_copy.calculate_hash();

        let mut mempool = self.transactions.write().await;

        // Vérifier si déjà présent
        if mempool.contains_key(&hash) {
            return Err(MempoolError::AlreadyExists);
        }

        // Vérifier la taille
        if mempool.len() >= self.max_size {
            // Retirer la transaction la plus ancienne
            self.evict_oldest(&mut mempool).await;
        }

        // Calculer la priorité (basée sur les frais)
        let priority = tx.fee / tx.amount.max(0.001);

        let entry = MempoolEntry {
            transaction: tx,
            received_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            priority,
        };

        mempool.insert(hash, entry);
        Ok(())
    }

    /// Obtenir les transactions pour un bloc (triées par priorité)
    pub async fn get_transactions_for_block(&self, max_count: usize) -> Vec<Transaction> {
        let mempool = self.transactions.read().await;
        
        // Nettoyer les transactions expirées
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut valid_entries: Vec<_> = mempool
            .values()
            .filter(|entry| now - entry.received_at < self.max_age)
            .collect();

        // Trier par priorité (décroissant)
        valid_entries.sort_by(|a, b| b.priority.partial_cmp(&a.priority).unwrap());

        // Prendre les N meilleures
        valid_entries
            .into_iter()
            .take(max_count)
            .map(|entry| entry.transaction.clone())
            .collect()
    }

    /// Retirer une transaction (après inclusion dans un bloc)
    pub async fn remove_transaction(&self, hash: &Hash) {
        let mut mempool = self.transactions.write().await;
        mempool.remove(hash);
    }

    /// Retirer plusieurs transactions
    pub async fn remove_transactions(&self, hashes: &[Hash]) {
        let mut mempool = self.transactions.write().await;
        for hash in hashes {
            mempool.remove(hash);
        }
    }

    /// Nettoyer les transactions expirées
    pub async fn cleanup_expired(&self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut mempool = self.transactions.write().await;
        mempool.retain(|_, entry| now - entry.received_at < self.max_age);
    }

    /// Retirer la transaction la plus ancienne
    async fn evict_oldest(&self, mempool: &mut HashMap<Hash, MempoolEntry>) {
        if let Some((oldest_hash, _)) = mempool
            .iter()
            .min_by_key(|(_, entry)| entry.received_at)
        {
            mempool.remove(oldest_hash);
        }
    }

    /// Obtenir la taille du mempool
    pub async fn size(&self) -> usize {
        self.transactions.read().await.len()
    }
}

/// Erreurs du mempool
#[derive(Debug, thiserror::Error)]
pub enum MempoolError {
    #[error("Invalid transaction")]
    InvalidTransaction,
    
    #[error("Transaction already exists")]
    AlreadyExists,
    
    #[error("Mempool is full")]
    Full,
}

