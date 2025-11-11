//! Gestion des wallets Mahala

use crate::PublicKey;
use crate::crypto::keys::KeyPair;

/// Adresse d'un wallet (alias pour PublicKey)
pub type WalletAddress = PublicKey;

/// Wallet Mahala
#[derive(Debug, Clone)]
pub struct Wallet {
    /// Paire de clés
    pub keypair: KeyPair,
    
    /// Adresse du wallet (clé publique)
    pub address: WalletAddress,
}

impl Wallet {
    /// Créer un nouveau wallet
    pub fn new() -> Self {
        let keypair = KeyPair::new();
        Self {
            address: *keypair.public_key(),
            keypair,
        }
    }

    /// Créer un wallet depuis une seed (déterministe)
    pub fn from_seed(seed: &[u8; 32]) -> Self {
        let keypair = KeyPair::from_seed(seed);
        Self {
            address: *keypair.public_key(),
            keypair,
        }
    }

    /// Obtenir l'adresse du wallet
    pub fn address(&self) -> &WalletAddress {
        &self.address
    }

    /// Obtenir la clé publique
    pub fn public_key(&self) -> &PublicKey {
        &self.address
    }

    /// Obtenir la clé privée (attention: sensible!)
    pub fn private_key(&self) -> &crate::PrivateKey {
        self.keypair.private_key()
    }
}

impl Default for Wallet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_creation() {
        let wallet = Wallet::new();
        assert_eq!(wallet.address.len(), 32);
    }

    #[test]
    fn test_wallet_from_seed() {
        let seed = [0u8; 32];
        let wallet1 = Wallet::from_seed(&seed);
        let wallet2 = Wallet::from_seed(&seed);
        
        assert_eq!(wallet1.address, wallet2.address);
    }
}

