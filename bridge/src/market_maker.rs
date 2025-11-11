//! Market Maker (AMM - Automated Market Maker) pour le bridge
//!
//! Utilise la formule Constant Product (x * y = k) comme Uniswap

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Market Maker avec réserves constantes
pub struct MarketMaker {
    /// Réserve June
    june_reserve: f64,
    
    /// Réserve Mahala
    mahala_reserve: f64,
    
    /// Produit constant (k = june * mahala)
    k: f64,
    
    /// Frais (0.1% = 0.001)
    fee: f64,
}

/// Quote pour un échange
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    /// Montant d'entrée
    pub input: f64,
    
    /// Montant de sortie
    pub output: f64,
    
    /// Frais
    pub fee: f64,
    
    /// Taux de change
    pub rate: f64,
}

/// Direction d'échange
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ExchangeDirection {
    /// Mahala vers June
    MahalaToJune,
    /// June vers Mahala
    JuneToMahala,
}

/// Requête d'échange
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeRequest {
    /// Direction
    pub direction: ExchangeDirection,
    
    /// Montant
    pub amount: f64,
    
    /// Adresse du wallet
    pub wallet_address: String,
}

/// Résultat d'échange
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeResult {
    /// Hash de la transaction
    pub tx_hash: String,
    
    /// Montant reçu
    pub amount_received: f64,
    
    /// Frais payés
    pub fee: f64,
}

#[derive(Debug, Error)]
pub enum MarketMakerError {
    #[error("Insufficient reserves")]
    InsufficientReserves,
    
    #[error("Amount too small")]
    AmountTooSmall,
    
    #[error("Slippage too high")]
    SlippageTooHigh,
}

impl MarketMaker {
    /// Créer un nouveau market maker
    pub fn new(june_reserve: f64, mahala_reserve: f64) -> Self {
        Self {
            june_reserve,
            mahala_reserve,
            k: june_reserve * mahala_reserve,
            fee: 0.001, // 0.1%
        }
    }

    /// Obtenir un devis pour Mahala → June
    pub fn quote_mahala_to_june(&self, mahala_input: f64) -> Result<Quote, MarketMakerError> {
        if mahala_input <= 0.0 {
            return Err(MarketMakerError::AmountTooSmall);
        }

        // Appliquer les frais
        let mahala_after_fee = mahala_input * (1.0 - self.fee);
        
        // Calculer avec la formule Constant Product
        // k = june * mahala
        // new_mahala = mahala_reserve + mahala_after_fee
        // new_june = k / new_mahala
        // june_output = june_reserve - new_june
        
        let new_mahala = self.mahala_reserve + mahala_after_fee;
        let new_june = self.k / new_mahala;
        let june_output = self.june_reserve - new_june;
        
        if june_output <= 0.0 {
            return Err(MarketMakerError::InsufficientReserves);
        }

        Ok(Quote {
            input: mahala_input,
            output: june_output,
            fee: mahala_input * self.fee,
            rate: june_output / mahala_input,
        })
    }

    /// Obtenir un devis pour June → Mahala
    pub fn quote_june_to_mahala(&self, june_input: f64) -> Result<Quote, MarketMakerError> {
        if june_input <= 0.0 {
            return Err(MarketMakerError::AmountTooSmall);
        }

        // Appliquer les frais
        let june_after_fee = june_input * (1.0 - self.fee);
        
        // Calculer avec la formule Constant Product
        let new_june = self.june_reserve + june_after_fee;
        let new_mahala = self.k / new_june;
        let mahala_output = self.mahala_reserve - new_mahala;
        
        if mahala_output <= 0.0 {
            return Err(MarketMakerError::InsufficientReserves);
        }

        Ok(Quote {
            input: june_input,
            output: mahala_output,
            fee: june_input * self.fee,
            rate: mahala_output / june_input,
        })
    }

    /// Exécuter un échange
    pub fn execute_exchange(
        &mut self,
        direction: ExchangeDirection,
        amount: f64,
    ) -> Result<ExchangeResult, MarketMakerError> {
        let quote = match direction {
            ExchangeDirection::MahalaToJune => self.quote_mahala_to_june(amount)?,
            ExchangeDirection::JuneToMahala => self.quote_june_to_mahala(amount)?,
        };

        // Mettre à jour les réserves
        match direction {
            ExchangeDirection::MahalaToJune => {
                self.mahala_reserve += amount * (1.0 - self.fee);
                self.june_reserve -= quote.output;
            }
            ExchangeDirection::JuneToMahala => {
                self.june_reserve += amount * (1.0 - self.fee);
                self.mahala_reserve -= quote.output;
            }
        }

        // Mettre à jour k (le produit constant change légèrement à cause des frais)
        self.k = self.june_reserve * self.mahala_reserve;

        // Générer un hash de transaction (simulé)
        let tx_hash = format!("0x{}", hex::encode(&[
            direction as u8,
            (amount as u64).to_le_bytes().as_slice(),
        ].concat()));

        Ok(ExchangeResult {
            tx_hash,
            amount_received: quote.output,
            fee: quote.fee,
        })
    }

    /// Obtenir les statistiques du market maker
    pub fn get_stats(&self) -> MarketMakerStats {
        MarketMakerStats {
            june_reserve: self.june_reserve,
            mahala_reserve: self.mahala_reserve,
            total_liquidity: self.k,
            fee_percentage: self.fee * 100.0,
        }
    }
}

/// Statistiques du market maker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketMakerStats {
    pub june_reserve: f64,
    pub mahala_reserve: f64,
    pub total_liquidity: f64,
    pub fee_percentage: f64,
}

