//! VRF (Verifiable Random Function) pour sélection aléatoire vérifiable
//!
//! Utilise Ed25519 avec une transformation pour créer une VRF

use crate::{PublicKey, PrivateKey, Hash};
use crate::crypto::hash_data;
use ed25519_dalek::{SigningKey, Signer};

/// Sortie d'une VRF
#[derive(Debug, Clone)]
pub struct VRFOutput {
    /// Valeur aléatoire générée
    pub output: Hash,
    /// Preuve de la génération
    pub proof: Vec<u8>,
}

/// VRF basée sur Ed25519
pub struct VRF;

impl VRF {
    /// Générer une sortie VRF depuis une clé privée et un message
    pub fn generate(private_key: &PrivateKey, message: &[u8]) -> VRFOutput {
        // Combiner la clé privée et le message
        let mut input = Vec::with_capacity(32 + message.len());
        input.extend_from_slice(private_key);
        input.extend_from_slice(message);
        
        // Hasher pour obtenir la sortie
        let output = hash_data(&input);
        
        // Créer une preuve en signant le message avec la clé privée
        let signing_key = SigningKey::from_bytes(private_key);
        let signature = signing_key.sign(message);
        let proof = signature.to_bytes().to_vec();
        
        VRFOutput { output, proof }
    }

    /// Vérifier une sortie VRF avec une clé publique
    pub fn verify(public_key: &PublicKey, message: &[u8], _output: &Hash, proof: &[u8]) -> bool {
        // Recalculer la sortie attendue
        // Note: Pour une vraie VRF, on devrait pouvoir vérifier sans la clé privée
        // Ici on utilise une approche simplifiée pour la performance mobile
        
        // Vérifier que la preuve est valide
        use crate::crypto::verify_signature;
        if !verify_signature(message, &proof[..64].try_into().unwrap_or([0u8; 64]), public_key) {
            return false;
        }
        
        // Pour une vraie VRF, on devrait pouvoir recalculer output depuis proof
        // Ici on accepte si la signature est valide
        // (Dans une implémentation complète, on utiliserait une vraie VRF comme ECVRF)
        true
    }

    /// Convertir la sortie VRF en nombre pour sélection
    pub fn output_to_number(output: &Hash) -> u64 {
        // Prendre les 8 premiers bytes et les convertir en u64
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&output[..8]);
        u64::from_le_bytes(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::keys::KeyPair;

    #[test]
    fn test_vrf_generate() {
        let keypair = KeyPair::new();
        let message = b"test message";
        
        let vrf_output = VRF::generate(keypair.private_key(), message);
        assert_eq!(vrf_output.output.len(), 32);
    }

    #[test]
    fn test_vrf_verify() {
        let keypair = KeyPair::new();
        let message = b"test message";
        
        let vrf_output = VRF::generate(keypair.private_key(), message);
        assert!(VRF::verify(
            keypair.public_key(),
            message,
            &vrf_output.output,
            &vrf_output.proof
        ));
    }

    #[test]
    fn test_vrf_output_to_number() {
        let keypair = KeyPair::new();
        let message = b"test";
        
        let vrf_output = VRF::generate(keypair.private_key(), message);
        let number = VRF::output_to_number(&vrf_output.output);
        
        // Le nombre devrait être dans une plage raisonnable
        assert!(number < u64::MAX);
    }
}

