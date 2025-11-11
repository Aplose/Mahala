//! Signatures cryptographiques Ed25519

use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use crate::{PublicKey, PrivateKey, Signature as MahalaSignature};

/// Signer des données avec une clé privée
pub fn sign(data: &[u8], private_key: &PrivateKey) -> MahalaSignature {
    let signing_key = SigningKey::from_bytes(private_key);
    let signature = signing_key.sign(data);
    signature.to_bytes()
}

/// Vérifier une signature
pub fn verify_signature(data: &[u8], signature: &MahalaSignature, public_key: &PublicKey) -> bool {
    let verifying_key = match VerifyingKey::from_bytes(public_key) {
        Ok(key) => key,
        Err(_) => return false,
    };
    
    // Signature::from_bytes prend une référence à [u8; 64]
    let sig = Signature::from_bytes(signature);
    
    verifying_key.verify(data, &sig).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::keys::KeyPair;

    #[test]
    fn test_sign_and_verify() {
        let keypair = KeyPair::new();
        let data = b"test message";
        
        let signature = sign(data, keypair.private_key());
        assert!(verify_signature(data, &signature, keypair.public_key()));
    }

    #[test]
    fn test_verify_fails_wrong_data() {
        let keypair = KeyPair::new();
        let data = b"test message";
        let wrong_data = b"wrong message";
        
        let signature = sign(data, keypair.private_key());
        assert!(!verify_signature(wrong_data, &signature, keypair.public_key()));
    }

    #[test]
    fn test_verify_fails_wrong_key() {
        let keypair1 = KeyPair::new();
        let keypair2 = KeyPair::new();
        let data = b"test message";
        
        let signature = sign(data, keypair1.private_key());
        assert!(!verify_signature(data, &signature, keypair2.public_key()));
    }
}


