//! Sécurité et limites pour le bridge

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;

/// Gestionnaire de sécurité
pub struct SecurityManager {
    /// Limites par utilisateur
    limits: Arc<RwLock<HashMap<String, UserLimits>>>,
    
    /// Limite quotidienne globale (en June)
    daily_limit: f64,
    
    /// Limite mensuelle globale (en June)
    monthly_limit: f64,
}

/// Limites d'un utilisateur
#[derive(Debug, Clone)]
struct UserLimits {
    /// Volume quotidien
    daily_volume: f64,
    
    /// Volume mensuel
    monthly_volume: f64,
    
    /// Dernière transaction
    last_transaction: u64,
    
    /// Dernière réinitialisation quotidienne
    last_daily_reset: u64,
    
    /// Dernière réinitialisation mensuelle
    last_monthly_reset: u64,
}

impl SecurityManager {
    /// Créer un nouveau gestionnaire de sécurité
    pub fn new(daily_limit: f64, monthly_limit: f64) -> Self {
        Self {
            limits: Arc::new(RwLock::new(HashMap::new())),
            daily_limit,
            monthly_limit,
        }
    }

    /// Vérifier si un échange est autorisé
    pub async fn check_exchange(
        &self,
        user_id: &str,
        amount: f64,
    ) -> Result<(), SecurityError> {
        let mut limits_guard = self.limits.write().await;
        
        // Obtenir ou créer les limites de l'utilisateur
        let limits = limits_guard
            .entry(user_id.to_string())
            .or_insert_with(|| UserLimits {
                daily_volume: 0.0,
                monthly_volume: 0.0,
                last_transaction: 0,
                last_daily_reset: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                last_monthly_reset: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            });

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Réinitialiser les compteurs si nécessaire
        if now - limits.last_daily_reset >= 86400 {
            limits.daily_volume = 0.0;
            limits.last_daily_reset = now;
        }

        if now - limits.last_monthly_reset >= 2592000 {
            limits.monthly_volume = 0.0;
            limits.last_monthly_reset = now;
        }

        // Vérifier les limites
        if limits.daily_volume + amount > self.daily_limit {
            return Err(SecurityError::DailyLimitExceeded);
        }

        if limits.monthly_volume + amount > self.monthly_limit {
            return Err(SecurityError::MonthlyLimitExceeded);
        }

        // Mettre à jour les volumes
        limits.daily_volume += amount;
        limits.monthly_volume += amount;
        limits.last_transaction = now;

        Ok(())
    }

    /// Obtenir les statistiques d'un utilisateur
    pub async fn get_user_stats(&self, user_id: &str) -> Option<UserStats> {
        let limits_guard = self.limits.read().await;
        limits_guard.get(user_id).map(|limits| UserStats {
            daily_volume: limits.daily_volume,
            monthly_volume: limits.monthly_volume,
            daily_limit: self.daily_limit,
            monthly_limit: self.monthly_limit,
        })
    }
}

/// Statistiques d'un utilisateur
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStats {
    pub daily_volume: f64,
    pub monthly_volume: f64,
    pub daily_limit: f64,
    pub monthly_limit: f64,
}

/// Erreurs de sécurité
#[derive(Debug, Error)]
pub enum SecurityError {
    #[error("Daily limit exceeded")]
    DailyLimitExceeded,
    
    #[error("Monthly limit exceeded")]
    MonthlyLimitExceeded,
    
    #[error("Amount too large")]
    AmountTooLarge,
}

use std::sync::Arc;
use thiserror::Error;

