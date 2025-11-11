//! Gestion des réserves du bridge

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Gestionnaire des réserves
pub struct Reserves {
    /// Réserves June
    june_reserve: Arc<RwLock<f64>>,
    
    /// Réserves Mahala
    mahala_reserve: Arc<RwLock<f64>>,
    
    /// Réserves initiales (pour référence)
    initial_june: f64,
    initial_mahala: f64,
}

impl Reserves {
    /// Créer de nouvelles réserves
    pub fn new(june: f64, mahala: f64) -> Self {
        Self {
            june_reserve: Arc::new(RwLock::new(june)),
            mahala_reserve: Arc::new(RwLock::new(mahala)),
            initial_june: june,
            initial_mahala: mahala,
        }
    }

    /// Obtenir les réserves June
    pub async fn get_june(&self) -> f64 {
        *self.june_reserve.read().await
    }

    /// Obtenir les réserves Mahala
    pub async fn get_mahala(&self) -> f64 {
        *self.mahala_reserve.read().await
    }

    /// Mettre à jour les réserves June
    pub async fn update_june(&self, amount: f64) {
        *self.june_reserve.write().await += amount;
    }

    /// Mettre à jour les réserves Mahala
    pub async fn update_mahala(&self, amount: f64) {
        *self.mahala_reserve.write().await += amount;
    }

    /// Obtenir les statistiques
    pub async fn get_stats(&self) -> ReserveStats {
        ReserveStats {
            june: self.get_june().await,
            mahala: self.get_mahala().await,
            initial_june: self.initial_june,
            initial_mahala: self.initial_mahala,
        }
    }
}

/// Statistiques des réserves
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReserveStats {
    pub june: f64,
    pub mahala: f64,
    pub initial_june: f64,
    pub initial_mahala: f64,
}

