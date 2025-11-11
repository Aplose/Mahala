//! Arbre de Merkle pour vérification SPV (Simplified Payment Verification)
//!
//! Permet de vérifier l'inclusion d'une transaction sans télécharger tout le bloc

use crate::{Hash, transaction::Transaction};
use crate::crypto::hash_data;

/// Arbre de Merkle pour les transactions
#[derive(Debug, Clone)]
pub struct MerkleTree {
    root: Hash,
    leaves: Vec<Hash>,
}

impl MerkleTree {
    /// Créer un arbre de Merkle depuis des transactions
    pub fn from_transactions(transactions: &[Transaction]) -> Self {
        if transactions.is_empty() {
            return Self {
                root: hash_data(b"empty"),
                leaves: Vec::new(),
            };
        }

        // Hasher chaque transaction
        let leaves: Vec<Hash> = transactions
            .iter()
            .map(|tx| {
                let mut tx_copy = tx.clone();
                tx_copy.calculate_hash();
                tx_copy.hash.unwrap()
            })
            .collect();

        let root = Self::build_tree(&leaves);
        
        Self { root, leaves }
    }

    /// Construire l'arbre de Merkle récursivement
    fn build_tree(hashes: &[Hash]) -> Hash {
        if hashes.len() == 1 {
            return hashes[0];
        }

        // Pairer les hash et les combiner
        let mut next_level = Vec::new();
        
        for i in (0..hashes.len()).step_by(2) {
            if i + 1 < hashes.len() {
                // Paire de deux hash
                let combined = Self::combine_hashes(&hashes[i], &hashes[i + 1]);
                next_level.push(combined);
            } else {
                // Hash impair: dupliquer le dernier
                let combined = Self::combine_hashes(&hashes[i], &hashes[i]);
                next_level.push(combined);
            }
        }

        Self::build_tree(&next_level)
    }

    /// Combiner deux hash
    fn combine_hashes(left: &Hash, right: &Hash) -> Hash {
        let mut combined = Vec::with_capacity(64);
        combined.extend_from_slice(left);
        combined.extend_from_slice(right);
        hash_data(&combined)
    }

    /// Obtenir la racine de l'arbre
    pub fn root(&self) -> Hash {
        self.root
    }

    /// Générer une preuve de Merkle pour une transaction
    pub fn generate_proof(&self, tx_hash: Hash) -> Option<MerkleProof> {
        // Trouver l'index de la transaction
        let index = self.leaves.iter().position(|&h| h == tx_hash)?;
        
        let mut proof = Vec::new();
        let mut current_index = index;
        let mut current_level = self.leaves.clone();

        while current_level.len() > 1 {
            let sibling_index = if current_index % 2 == 0 {
                current_index + 1
            } else {
                current_index - 1
            };

            if sibling_index < current_level.len() {
                proof.push(current_level[sibling_index]);
            } else {
                // Pas de sibling: dupliquer
                proof.push(current_level[current_index]);
            }

            // Monter d'un niveau
            let mut next_level = Vec::new();
            for i in (0..current_level.len()).step_by(2) {
                if i + 1 < current_level.len() {
                    next_level.push(Self::combine_hashes(&current_level[i], &current_level[i + 1]));
                } else {
                    next_level.push(Self::combine_hashes(&current_level[i], &current_level[i]));
                }
            }

            current_level = next_level;
            current_index /= 2;
        }

        Some(MerkleProof {
            leaf_hash: tx_hash,
            path: proof,
            index,
        })
    }

    /// Vérifier une preuve de Merkle
    pub fn verify_proof(proof: &MerkleProof, root: Hash) -> bool {
        let mut current_hash = proof.leaf_hash;
        let mut current_index = proof.index;

        for sibling in &proof.path {
            if current_index % 2 == 0 {
                current_hash = Self::combine_hashes(&current_hash, sibling);
            } else {
                current_hash = Self::combine_hashes(sibling, &current_hash);
            }
            current_index /= 2;
        }

        current_hash == root
    }
}

/// Preuve de Merkle pour une transaction
#[derive(Debug, Clone)]
pub struct MerkleProof {
    /// Hash de la transaction
    pub leaf_hash: Hash,
    /// Chemin vers la racine (siblings)
    pub path: Vec<Hash>,
    /// Index de la transaction
    pub index: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::keys::KeyPair;

    #[test]
    fn test_merkle_tree_empty() {
        let tree = MerkleTree::from_transactions(&[]);
        assert_ne!(tree.root(), [0u8; 32]);
    }

    #[test]
    fn test_merkle_tree_single() {
        let keypair1 = KeyPair::new();
        let keypair2 = KeyPair::new();
        
        let mut tx = crate::transaction::Transaction::new(
            *keypair1.public_key(),
            *keypair2.public_key(),
            100.0,
            0.1,
            Default::default(),
        );
        tx.sign(keypair1.private_key()).unwrap();
        tx.calculate_hash();
        
        let tree = MerkleTree::from_transactions(&[tx.clone()]);
        let root = tree.root();
        
        // Vérifier la preuve
        let proof = tree.generate_proof(tx.hash.unwrap()).unwrap();
        assert!(MerkleTree::verify_proof(&proof, root));
    }

    #[test]
    fn test_merkle_proof_multiple() {
        let keypair1 = KeyPair::new();
        let keypair2 = KeyPair::new();
        
        let mut txs = Vec::new();
        for i in 0..4 {
            let mut tx = crate::transaction::Transaction::new(
                *keypair1.public_key(),
                *keypair2.public_key(),
                100.0 + i as f64,
                0.1,
                Default::default(),
            );
            tx.sign(keypair1.private_key()).unwrap();
            tx.calculate_hash();
            txs.push(tx);
        }
        
        let tree = MerkleTree::from_transactions(&txs);
        let root = tree.root();
        
        // Vérifier chaque transaction
        for tx in &txs {
            let proof = tree.generate_proof(tx.hash.unwrap()).unwrap();
            assert!(MerkleTree::verify_proof(&proof, root));
        }
    }
}

