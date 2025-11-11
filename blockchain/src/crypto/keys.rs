//! Gestion des clés cryptographiques Ed25519

use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
use crate::{PublicKey, PrivateKey};

/// Paire de clés cryptographiques
#[derive(Debug, Clone)]
pub struct KeyPair {
    pub public_key: PublicKey,
    pub private_key: PrivateKey,
}

impl KeyPair {
    /// Créer une nouvelle paire de clés
    pub fn new() -> Self {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();
        
        Self {
            public_key: verifying_key.to_bytes(),
            private_key: signing_key.to_bytes(),
        }
    }

    /// Créer une paire de clés depuis une seed (déterministe)
    pub fn from_seed(seed: &[u8; 32]) -> Self {
        let signing_key = SigningKey::from_bytes(seed);
        let verifying_key = signing_key.verifying_key();
        
        Self {
            public_key: verifying_key.to_bytes(),
            private_key: signing_key.to_bytes(),
        }
    }

    /// Obtenir la clé publique
    pub fn public_key(&self) -> &PublicKey {
        &self.public_key
    }

    /// Obtenir la clé privée (attention: sensible!)
    pub fn private_key(&self) -> &PrivateKey {
        &self.private_key
    }
}

impl Default for KeyPair {
    fn default() -> Self {
        Self::new()
    }
}

/// Générer une nouvelle paire de clés
pub fn generate_keypair() -> KeyPair {
    KeyPair::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_generation() {
        let keypair = KeyPair::new();
        assert_eq!(keypair.public_key.len(), 32);
        assert_eq!(keypair.private_key.len(), 32);
    }

    #[test]
    fn test_keypair_from_seed() {
        let seed = [0u8; 32];
        let keypair1 = KeyPair::from_seed(&seed);
        let keypair2 = KeyPair::from_seed(&seed);
        
        // Même seed = même clés
        assert_eq!(keypair1.public_key, keypair2.public_key);
        assert_eq!(keypair1.private_key, keypair2.private_key);
    }
}


