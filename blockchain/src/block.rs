//! Structure des blocs de la blockchain Mahala

use serde::{Deserialize, Serialize};
use crate::{Hash, Signature, PublicKey, Timestamp, Amount};
use crate::transaction::Transaction;
use crate::crypto::{hash_data, hash_to_string};
use crate::storage::merkle::MerkleTree;

/// En-tête d'un bloc
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    /// Numéro de bloc (hauteur)
    pub height: u64,
    
    /// Hash du bloc précédent
    pub previous_hash: Hash,
    
    /// Racine de Merkle des transactions
    pub merkle_root: Hash,
    
    /// Timestamp de création
    pub timestamp: Timestamp,
    
    /// Hash du validateur qui a créé ce bloc
    pub validator: PublicKey,
    
    /// Version du protocole
    pub version: u32,
}

/// Bloc de la blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    /// En-tête du bloc
    pub header: BlockHeader,
    
    /// Transactions dans ce bloc
    pub transactions: Vec<Transaction>,
    
    /// Signatures des validateurs (quorum 67%)
    pub validator_signatures: Vec<ValidatorSignature>,
    
    /// Hash du bloc (calculé)
    #[serde(skip)]
    pub hash: Option<Hash>,
}

/// Signature d'un validateur
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorSignature {
    /// Clé publique du validateur (sérialisée en hex)
    #[serde(with = "hex_serde")]
    pub validator: PublicKey,
    
    /// Signature du bloc (sérialisée en hex, 64 bytes)
    #[serde(serialize_with = "serialize_signature", deserialize_with = "deserialize_signature")]
    pub signature: Signature,
}

fn serialize_signature<S>(bytes: &[u8; 64], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let hex = hex::encode(bytes);
    hex.serialize(serializer)
}

fn deserialize_signature<'de, D>(deserializer: D) -> Result<[u8; 64], D::Error>
where
    D: serde::Deserializer<'de>,
{
    let hex_str = String::deserialize(deserializer)?;
    let bytes = hex::decode(&hex_str)
        .map_err(serde::de::Error::custom)?;
    if bytes.len() != 64 {
        return Err(serde::de::Error::custom("Invalid length for Signature"));
    }
    let mut result = [0u8; 64];
    result.copy_from_slice(&bytes);
    Ok(result)
}

/// Helper pour sérialiser/désérialiser les arrays en hex
mod hex_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(bytes: &[u8; 32], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let hex = hex::encode(bytes);
        hex.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<[u8; 32], D::Error>
    where
        D: Deserializer<'de>,
    {
        let hex_str = String::deserialize(deserializer)?;
        let bytes = hex::decode(&hex_str)
            .map_err(serde::de::Error::custom)?;
        if bytes.len() != 32 {
            return Err(serde::de::Error::custom("Invalid length for PublicKey"));
        }
        let mut result = [0u8; 32];
        result.copy_from_slice(&bytes);
        Ok(result)
    }
}

impl BlockHeader {
    /// Créer un nouvel en-tête de bloc
    pub fn new(
        height: u64,
        previous_hash: Hash,
        merkle_root: Hash,
        validator: PublicKey,
    ) -> Self {
        Self {
            height,
            previous_hash,
            merkle_root,
            timestamp: chrono::Utc::now().timestamp(),
            validator,
            version: 1,
        }
    }

    /// Sérialiser l'en-tête pour le hash
    fn serialize_for_hash(&self) -> Vec<u8> {
        serde_json::to_vec(&HeaderForHash {
            height: self.height,
            previous_hash: &self.previous_hash,
            merkle_root: &self.merkle_root,
            timestamp: self.timestamp,
            validator: &self.validator,
            version: self.version,
        }).expect("Serialization should never fail")
    }
}

/// Structure pour le calcul du hash (sans signatures)
#[derive(Serialize)]
struct HeaderForHash<'a> {
    height: u64,
    previous_hash: &'a Hash,
    merkle_root: &'a Hash,
    timestamp: Timestamp,
    validator: &'a PublicKey,
    version: u32,
}

impl Block {
    /// Créer un nouveau bloc
    pub fn new(
        height: u64,
        previous_hash: Hash,
        transactions: Vec<Transaction>,
        validator: PublicKey,
    ) -> Self {
        // Calculer la racine de Merkle
        let merkle_root = if transactions.is_empty() {
            // Bloc vide: hash spécial
            hash_data(b"empty_block")
        } else {
            MerkleTree::from_transactions(&transactions).root()
        };

        let header = BlockHeader::new(
            height,
            previous_hash,
            merkle_root,
            validator,
        );

        Self {
            header,
            transactions,
            validator_signatures: Vec::new(),
            hash: None,
        }
    }

    /// Calculer le hash du bloc
    pub fn calculate_hash(&mut self) -> Hash {
        let data = self.header.serialize_for_hash();
        let hash = hash_data(&data);
        self.hash = Some(hash);
        hash
    }

    /// Ajouter une signature de validateur
    pub fn add_validator_signature(&mut self, validator: PublicKey, signature: Signature) {
        // Vérifier qu'on n'a pas déjà cette signature
        if !self.validator_signatures.iter().any(|vs| vs.validator == validator) {
            self.validator_signatures.push(ValidatorSignature {
                validator,
                signature,
            });
        }
    }

    /// Vérifier les signatures des validateurs
    fn verify_validator_signatures(&self, block_hash: Hash) -> bool {
        for vs in &self.validator_signatures {
            use crate::crypto::verify_signature;
            if !verify_signature(&block_hash, &vs.signature, &vs.validator) {
                return false;
            }
        }
        true
    }

    /// Vérifier si le bloc a atteint le quorum (67% des validateurs)
    pub fn has_quorum(&self, total_validators: usize) -> bool {
        let required = (total_validators * 67 + 99) / 100; // Arrondi supérieur de 67%
        self.validator_signatures.len() >= required
    }

    /// Vérifier la validité du bloc
    pub fn is_valid(&self, previous_block_hash: Option<Hash>) -> bool {
        // Vérifier le hash précédent
        if let Some(prev_hash) = previous_block_hash {
            if self.header.previous_hash != prev_hash {
                return false;
            }
        }

        // Vérifier la hauteur
        if self.header.height == 0 && !previous_block_hash.is_none() {
            return false;
        }

        // Vérifier les transactions
        for tx in &self.transactions {
            if !tx.is_valid() {
                return false;
            }
        }

        // Vérifier la racine de Merkle
        let calculated_root = if self.transactions.is_empty() {
            hash_data(b"empty_block")
        } else {
            MerkleTree::from_transactions(&self.transactions).root()
        };

        if self.header.merkle_root != calculated_root {
            return false;
        }

        // Vérifier les signatures des validateurs
        let block_hash = self.hash.expect("Block hash should be calculated");
        if !self.verify_validator_signatures(block_hash) {
            return false;
        }

        true
    }

    /// Obtenir le hash du bloc (calculé si nécessaire)
    pub fn hash(&mut self) -> Hash {
        if let Some(hash) = self.hash {
            hash
        } else {
            self.calculate_hash()
        }
    }

    /// Obtenir le hash du bloc (sans mutation)
    pub fn get_hash(&self) -> Option<Hash> {
        self.hash
    }

    /// Obtenir le hash en string
    pub fn hash_string(&mut self) -> String {
        let hash = self.hash();
        hash_to_string(&hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::keys::KeyPair;

    #[test]
    fn test_block_creation() {
        let keypair = KeyPair::new();
        let transactions = Vec::new();
        let previous_hash = hash_data(b"genesis");
        
        let mut block = Block::new(1, previous_hash, transactions, *keypair.public_key());
        let hash = block.calculate_hash();
        
        assert_eq!(block.header.height, 1);
        assert_eq!(block.header.previous_hash, previous_hash);
        assert!(block.is_valid(Some(previous_hash)));
    }

    #[test]
    fn test_block_merkle_root() {
        let keypair = KeyPair::new();
        let keypair1 = KeyPair::new();
        let keypair2 = KeyPair::new();
        
        let mut tx1 = crate::transaction::Transaction::new(
            *keypair1.public_key(),
            *keypair2.public_key(),
            100.0,
            0.1,
            Default::default(),
        );
        tx1.sign(keypair1.private_key()).unwrap();
        
        let transactions = vec![tx1];
        let previous_hash = hash_data(b"test");
        
        let block = Block::new(0, previous_hash, transactions.clone(), *keypair.public_key());
        
        // Vérifier que la racine de Merkle est calculée
        assert_ne!(block.header.merkle_root, [0u8; 32]);
    }

    #[test]
    fn test_block_quorum() {
        let keypair = KeyPair::new();
        let block = Block::new(0, hash_data(b"genesis"), Vec::new(), *keypair.public_key());
        
        // Avec 10 validateurs, il faut 7 signatures (67% = 6.7, arrondi à 7)
        assert!(!block.has_quorum(10));
        
        // Avec 3 validateurs, il faut 2 signatures (67% de 3 = 2.01, arrondi à 3)
        // En fait, avec l'arrondi supérieur: (3 * 67 + 99) / 100 = (201 + 99) / 100 = 300 / 100 = 3
        let mut block2 = Block::new(0, hash_data(b"genesis"), Vec::new(), *keypair.public_key());
        // Ajouter 3 signatures pour atteindre le quorum
        let keypair2 = KeyPair::new();
        let keypair3 = KeyPair::new();
        block2.add_validator_signature(*keypair.public_key(), [0u8; 64]);
        block2.add_validator_signature(*keypair2.public_key(), [0u8; 64]);
        block2.add_validator_signature(*keypair3.public_key(), [0u8; 64]);
        assert!(block2.has_quorum(3));
        
        // Avec 2 signatures sur 3, ça ne devrait pas suffire
        let mut block3 = Block::new(0, hash_data(b"genesis"), Vec::new(), *keypair.public_key());
        block3.add_validator_signature(*keypair.public_key(), [0u8; 64]);
        block3.add_validator_signature(*keypair2.public_key(), [0u8; 64]);
        assert!(!block3.has_quorum(3));
    }
}

