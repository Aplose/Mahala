//! Structures et logique des transactions Mahala

use serde::{Deserialize, Serialize};
use crate::{Hash, PublicKey, Amount, Timestamp};
use crate::crypto::{hash_data, sign, verify_signature};

/// Transaction sur la blockchain Mahala
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Hash de la transaction (calculé)
    #[serde(skip)]
    pub hash: Option<Hash>,
    
    /// Adresse expéditeur (clé publique)
    pub from: PublicKey,
    
    /// Adresse destinataire (clé publique)
    pub to: PublicKey,
    
    /// Montant en Mahala
    pub amount: Amount,
    
    /// Frais de transaction
    pub fee: Amount,
    
    /// Timestamp de création
    pub timestamp: Timestamp,
    
    /// Données supplémentaires (métadonnées)
    #[serde(default)]
    pub metadata: TransactionMetadata,
    
    /// Signature de la transaction (sérialisée en hex)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub signature: Option<String>,
}

/// Métadonnées optionnelles pour les transactions
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionMetadata {
    /// Type de transaction
    #[serde(default)]
    pub transaction_type: TransactionType,
    
    /// Données additionnelles (JSON)
    #[serde(default)]
    pub extra_data: Option<String>,
}

/// Types de transactions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransactionType {
    /// Transfert simple
    Transfer,
    /// Paiement de location
    RentalPayment,
    /// Achat dans boutique
    ShopPurchase,
    /// Téléportation
    Teleport,
    /// Distribution DU
    UniversalDividend,
    /// Transaction NFT
    NFT,
}

impl Default for TransactionType {
    fn default() -> Self {
        Self::Transfer
    }
}

impl Transaction {
    /// Créer une nouvelle transaction
    pub fn new(
        from: PublicKey,
        to: PublicKey,
        amount: Amount,
        fee: Amount,
        metadata: TransactionMetadata,
    ) -> Self {
        let timestamp = chrono::Utc::now().timestamp();
        
        Self {
            hash: None,
            from,
            to,
            amount,
            fee,
            timestamp,
            metadata,
            signature: None,
        }
    }

    /// Calculer le hash de la transaction
    pub fn calculate_hash(&mut self) -> Hash {
        // Exclure hash et signature du calcul
        let data = serde_json::to_vec(&TransactionForHash {
            from: &self.from,
            to: &self.to,
            amount: self.amount,
            fee: self.fee,
            timestamp: self.timestamp,
            metadata: &self.metadata,
        }).expect("Serialization should never fail");
        
        let hash = hash_data(&data);
        self.hash = Some(hash);
        hash
    }

    /// Signer la transaction avec une clé privée
    pub fn sign(&mut self, private_key: &crate::PrivateKey) -> Result<(), String> {
        // Calculer le hash d'abord
        let hash = self.calculate_hash();
        
        // Signer le hash
        let signature = sign(&hash, private_key);
        self.signature = Some(hex::encode(signature));
        
        Ok(())
    }

    /// Vérifier la signature de la transaction
    pub fn verify_signature(&self) -> bool {
        let Some(sig_hex) = &self.signature else {
            return false;
        };
        
        let signature_bytes = match hex::decode(sig_hex) {
            Ok(bytes) if bytes.len() == 64 => {
                let mut sig = [0u8; 64];
                sig.copy_from_slice(&bytes);
                sig
            }
            _ => return false,
        };
        
        // Recalculer le hash pour vérification
        let mut tx_copy = self.clone();
        tx_copy.signature = None; // Exclure la signature du calcul
        let hash = tx_copy.calculate_hash();
        
        verify_signature(&hash, &signature_bytes, &self.from)
    }

    /// Vérifier la validité de la transaction
    pub fn is_valid(&self) -> bool {
        // Vérifier signature
        if !self.verify_signature() {
            return false;
        }
        
        // Vérifier montants positifs
        if self.amount <= 0.0 || self.fee < 0.0 {
            return false;
        }
        
        // Vérifier que from != to (sauf pour DU)
        if self.from == self.to && self.metadata.transaction_type != TransactionType::UniversalDividend {
            return false;
        }
        
        true
    }
}

/// Structure pour le calcul du hash (sans signature)
#[derive(Serialize)]
struct TransactionForHash<'a> {
    from: &'a PublicKey,
    to: &'a PublicKey,
    amount: Amount,
    fee: Amount,
    timestamp: Timestamp,
    metadata: &'a TransactionMetadata,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::keys::KeyPair;

    #[test]
    fn test_transaction_creation() {
        let keypair1 = KeyPair::new();
        let keypair2 = KeyPair::new();
        
        let mut tx = Transaction::new(
            *keypair1.public_key(),
            *keypair2.public_key(),
            100.0,
            0.1,
            TransactionMetadata::default(),
        );
        
        tx.sign(keypair1.private_key()).unwrap();
        assert!(tx.is_valid());
    }

    #[test]
    fn test_transaction_hash_consistency() {
        let keypair1 = KeyPair::new();
        let keypair2 = KeyPair::new();
        
        let mut tx1 = Transaction::new(
            *keypair1.public_key(),
            *keypair2.public_key(),
            100.0,
            0.1,
            TransactionMetadata::default(),
        );
        
        let mut tx2 = tx1.clone();
        
        let hash1 = tx1.calculate_hash();
        let hash2 = tx2.calculate_hash();
        
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_transaction_invalid_negative_amount() {
        let keypair1 = KeyPair::new();
        let keypair2 = KeyPair::new();
        
        let mut tx = Transaction::new(
            *keypair1.public_key(),
            *keypair2.public_key(),
            -10.0,
            0.1,
            TransactionMetadata::default(),
        );
        
        tx.sign(keypair1.private_key()).unwrap();
        assert!(!tx.is_valid());
    }
}


