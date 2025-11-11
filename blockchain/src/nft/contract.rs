//! Smart Contract NFT pour Mahala
//!
//! Implémentation simplifiée d'un contrat NFT pour la blockchain Mahala

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::{PublicKey, Amount};
use crate::crypto::hash_data;
use thiserror::Error;

/// Type de NFT
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NFTType {
    /// Art numérique
    Art,
    /// Musique
    Music,
    /// Vidéo
    Video,
    /// Modèle 3D pour Luanti
    Model3D,
    /// Terrain virtuel
    VirtualLand,
    /// Contrat de location
    RentalContract,
    /// Bon pour service réel
    ServiceTicket,
    /// Bon d'achat produit
    ProductVoucher,
}

/// Métadonnées d'un NFT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFTMetadata {
    /// Nom du NFT
    pub name: String,
    /// Description
    pub description: String,
    /// Type de NFT
    pub nft_type: NFTType,
    /// URL du média (IPFS hash ou URL)
    pub media_url: String,
    /// URL de la miniature
    pub thumbnail_url: String,
    /// Attributs additionnels
    #[serde(default)]
    pub attributes: HashMap<String, String>,
}

/// NFT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFT {
    /// ID unique du NFT
    pub id: String,
    /// Propriétaire actuel
    pub owner: PublicKey,
    /// Créateur original
    pub creator: PublicKey,
    /// Métadonnées
    pub metadata: NFTMetadata,
    /// Date de création
    pub created_at: i64,
    /// Pourcentage de royalties pour le créateur (max 10%)
    pub royalty_percentage: u8,
}

/// Listing d'un NFT en vente
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFTListing {
    /// ID du NFT
    pub nft_id: String,
    /// Vendeur
    pub seller: PublicKey,
    /// Prix en Mahala
    pub price: Amount,
    /// Date de mise en vente
    pub listed_at: i64,
    /// Date d'expiration (optionnel)
    pub expires_at: Option<i64>,
}

/// Résultat d'un transfert NFT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFTTransferResult {
    /// ID du NFT
    pub nft_id: String,
    /// Nouveau propriétaire
    pub new_owner: PublicKey,
    /// Vendeur
    pub seller: PublicKey,
    /// Créateur
    pub creator: PublicKey,
    /// Montant total payé
    pub total_paid: Amount,
    /// Montant reçu par le vendeur
    pub seller_receives: Amount,
    /// Montant reçu par le créateur (royalties)
    pub creator_receives: Amount,
}

/// Erreurs du contrat NFT
#[derive(Debug, Error)]
pub enum NFTError {
    #[error("NFT not found")]
    NotFound,
    #[error("Not the owner")]
    NotOwner,
    #[error("Royalty percentage too high (max 10%)")]
    RoyaltyTooHigh,
    #[error("NFT already listed")]
    AlreadyListed,
    #[error("NFT not listed for sale")]
    NotListed,
    #[error("Listing expired")]
    ListingExpired,
    #[error("Insufficient payment")]
    InsufficientPayment,
}

/// Contrat NFT
pub struct NFTContract {
    /// NFTs indexés par ID
    nfts: HashMap<String, NFT>,
    /// Listings actifs
    listings: HashMap<String, NFTListing>,
}

impl NFTContract {
    /// Créer un nouveau contrat NFT
    pub fn new() -> Self {
        Self {
            nfts: HashMap::new(),
            listings: HashMap::new(),
        }
    }

    /// Minter un nouveau NFT
    pub fn mint(
        &mut self,
        creator: PublicKey,
        metadata: NFTMetadata,
        royalty_percentage: u8,
    ) -> Result<String, NFTError> {
        if royalty_percentage > 10 {
            return Err(NFTError::RoyaltyTooHigh);
        }

        // Générer un ID unique
        let nft_id = Self::generate_nft_id(&creator, &metadata);
        
        let nft = NFT {
            id: nft_id.clone(),
            owner: creator,
            creator,
            metadata,
            created_at: chrono::Utc::now().timestamp(),
            royalty_percentage,
        };

        self.nfts.insert(nft_id.clone(), nft);
        Ok(nft_id)
    }

    /// Lister un NFT à la vente
    pub fn list_for_sale(
        &mut self,
        nft_id: &str,
        seller: &PublicKey,
        price: Amount,
        duration_days: Option<u64>,
    ) -> Result<(), NFTError> {
        let nft = self.nfts.get(nft_id)
            .ok_or(NFTError::NotFound)?;

        if &nft.owner != seller {
            return Err(NFTError::NotOwner);
        }

        if self.listings.contains_key(nft_id) {
            return Err(NFTError::AlreadyListed);
        }

        let expires_at = duration_days.map(|days| {
            chrono::Utc::now().timestamp() + (days as i64 * 86400)
        });

        let listing = NFTListing {
            nft_id: nft_id.to_string(),
            seller: *seller,
            price,
            listed_at: chrono::Utc::now().timestamp(),
            expires_at,
        };

        self.listings.insert(nft_id.to_string(), listing);
        Ok(())
    }

    /// Acheter un NFT
    pub fn buy_nft(
        &mut self,
        nft_id: &str,
        buyer: &PublicKey,
        payment: Amount,
    ) -> Result<NFTTransferResult, NFTError> {
        let listing = self.listings.get(nft_id)
            .ok_or(NFTError::NotListed)?;

        // Vérifier expiration
        if let Some(expires_at) = listing.expires_at {
            if chrono::Utc::now().timestamp() > expires_at {
                self.listings.remove(nft_id);
                return Err(NFTError::ListingExpired);
            }
        }

        // Vérifier prix
        if payment < listing.price {
            return Err(NFTError::InsufficientPayment);
        }

        let nft = self.nfts.get_mut(nft_id)
            .ok_or(NFTError::NotFound)?;

        // Calculer royalties
        let royalty_amount = (payment * nft.royalty_percentage as f64) / 100.0;
        let seller_amount = payment - royalty_amount;

        // Transférer NFT
        nft.owner = *buyer;

        // Retirer listing (doit être fait avant d'utiliser listing)
        let seller = listing.seller;
        let creator = nft.creator;
        self.listings.remove(nft_id);

        Ok(NFTTransferResult {
            nft_id: nft_id.to_string(),
            new_owner: *buyer,
            seller,
            creator,
            total_paid: payment,
            seller_receives: seller_amount,
            creator_receives: royalty_amount,
        })
    }

    /// Transférer un NFT (gratuit)
    pub fn transfer(
        &mut self,
        nft_id: &str,
        from: &PublicKey,
        to: &PublicKey,
    ) -> Result<(), NFTError> {
        let nft = self.nfts.get_mut(nft_id)
            .ok_or(NFTError::NotFound)?;

        if &nft.owner != from {
            return Err(NFTError::NotOwner);
        }

        // Retirer listing si existe
        self.listings.remove(nft_id);

        nft.owner = *to;
        Ok(())
    }

    /// Obtenir les NFTs d'un propriétaire
    pub fn get_nfts_by_owner(&self, owner: &PublicKey) -> Vec<&NFT> {
        self.nfts.values()
            .filter(|nft| &nft.owner == owner)
            .collect()
    }

    /// Obtenir les listings actifs
    pub fn get_active_listings(&self) -> Vec<(&NFT, &NFTListing)> {
        let now = chrono::Utc::now().timestamp();

        self.listings.values()
            .filter(|listing| {
                listing.expires_at.map_or(true, |exp| exp > now)
            })
            .filter_map(|listing| {
                self.nfts.get(&listing.nft_id)
                    .map(|nft| (nft, listing))
            })
            .collect()
    }

    /// Obtenir un NFT par ID
    pub fn get_nft(&self, nft_id: &str) -> Option<&NFT> {
        self.nfts.get(nft_id)
    }

    /// Générer un ID unique pour un NFT
    fn generate_nft_id(creator: &PublicKey, metadata: &NFTMetadata) -> String {
        let mut data = Vec::new();
        data.extend_from_slice(creator);
        data.extend_from_slice(metadata.name.as_bytes());
        let timestamp = chrono::Utc::now().timestamp();
        data.extend_from_slice(&timestamp.to_le_bytes());
        
        let hash = hash_data(&data);
        hex::encode(&hash[..16]) // Utiliser les 16 premiers bytes pour l'ID
    }
}

impl Default for NFTContract {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::keys::KeyPair;

    #[test]
    fn test_nft_mint() {
        let mut contract = NFTContract::new();
        let keypair = KeyPair::new();
        
        let metadata = NFTMetadata {
            name: "Test NFT".to_string(),
            description: "A test NFT".to_string(),
            nft_type: NFTType::Art,
            media_url: "ipfs://test".to_string(),
            thumbnail_url: "ipfs://thumb".to_string(),
            attributes: HashMap::new(),
        };

        let result = contract.mint(*keypair.public_key(), metadata, 5);
        assert!(result.is_ok());
    }

    #[test]
    fn test_nft_list_and_buy() {
        let mut contract = NFTContract::new();
        let creator = KeyPair::new();
        let buyer = KeyPair::new();
        
        let metadata = NFTMetadata {
            name: "Test NFT".to_string(),
            description: "A test NFT".to_string(),
            nft_type: NFTType::Art,
            media_url: "ipfs://test".to_string(),
            thumbnail_url: "ipfs://thumb".to_string(),
            attributes: HashMap::new(),
        };

        let nft_id = contract.mint(*creator.public_key(), metadata, 5).unwrap();
        
        // Lister
        contract.list_for_sale(&nft_id, creator.public_key(), 100.0, None).unwrap();
        
        // Acheter
        let result = contract.buy_nft(&nft_id, buyer.public_key(), 100.0);
        assert!(result.is_ok());
        
        let transfer = result.unwrap();
        assert_eq!(transfer.seller_receives, 95.0); // 100 - 5% royalties
        assert_eq!(transfer.creator_receives, 5.0);
    }
}

