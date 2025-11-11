//! Calculateur du Dividende Universel selon la TRM
//!
//! Formule: DU(t) = c * M(t) / N(t)
//! où:
//! - c = 4.88% par semestre (taux de croissance)
//! - M(t) = masse monétaire au temps t
//! - N(t) = nombre de membres au temps t

use crate::{Amount, Timestamp};
use chrono::{DateTime, Utc};

/// Configuration du DU
#[derive(Debug, Clone)]
pub struct DUConfig {
    /// Taux de croissance par semestre (c = 4.88%)
    pub growth_rate_per_semester: f64,
    
    /// Durée d'un semestre en jours (183 jours)
    pub semester_days: i64,
    
    /// Date de création de la monnaie (genesis)
    pub genesis_date: DateTime<Utc>,
}

impl Default for DUConfig {
    fn default() -> Self {
        Self {
            growth_rate_per_semester: 0.0488, // 4.88%
            semester_days: 183,
            genesis_date: Utc::now(), // À ajuster selon le lancement réel
        }
    }
}

/// Calculateur du Dividende Universel
pub struct DUCalculator {
    config: DUConfig,
}

impl DUCalculator {
    /// Créer un nouveau calculateur DU
    pub fn new(config: DUConfig) -> Self {
        Self { config }
    }

    /// Calculer le DU pour une date donnée
    pub fn calculate_du(
        &self,
        current_mass: Amount,
        member_count: u64,
        timestamp: Timestamp,
    ) -> Amount {
        if member_count == 0 {
            return 0.0;
        }

        // Calculer le nombre de semestres écoulés depuis la genèse
        let genesis_ts = self.config.genesis_date.timestamp();
        let days_since_genesis = (timestamp - genesis_ts) / 86400; // secondes -> jours
        
        if days_since_genesis < 0 {
            return 0.0;
        }

        // Calculer le DU selon la formule TRM
        // DU(t) = c * M(t) / N(t)
        let du = self.config.growth_rate_per_semester * current_mass / (member_count as f64);
        
        // Ajuster pour la distribution quotidienne
        // Le DU semestriel est distribué quotidiennement
        let daily_du = du / (self.config.semester_days as f64);
        
        daily_du
    }

    /// Calculer le DU quotidien actuel
    pub fn calculate_current_du(&self, current_mass: Amount, member_count: u64) -> Amount {
        let now = Utc::now().timestamp();
        self.calculate_du(current_mass, member_count, now)
    }

    /// Vérifier si une réévaluation est nécessaire
    pub fn should_reevaluate(&self, last_reevaluation: Timestamp, current_timestamp: Timestamp) -> bool {
        let days_since = (current_timestamp - last_reevaluation) / 86400;
        days_since >= self.config.semester_days
    }

    /// Calculer le nombre de jours jusqu'à la prochaine réévaluation
    pub fn days_until_reevaluation(&self, last_reevaluation: Timestamp, current_timestamp: Timestamp) -> i64 {
        let days_since = (current_timestamp - last_reevaluation) / 86400;
        (self.config.semester_days - days_since).max(0)
    }

    /// Calculer la masse monétaire théorique après un semestre
    pub fn projected_mass_after_semester(&self, current_mass: Amount) -> Amount {
        current_mass * (1.0 + self.config.growth_rate_per_semester)
    }

    /// Calculer le DU cumulé sur un semestre
    pub fn semester_du_total(&self, current_mass: Amount, member_count: u64) -> Amount {
        if member_count == 0 {
            return 0.0;
        }
        
        self.config.growth_rate_per_semester * current_mass / (member_count as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_du_calculation() {
        let config = DUConfig::default();
        let calculator = DUCalculator::new(config);
        
        let current_mass = 1_000_000.0; // 1M Mahala
        let member_count = 1000; // 1000 membres
        
        let du = calculator.calculate_current_du(current_mass, member_count);
        
        // Le DU devrait être positif
        assert!(du > 0.0);
        
        // Vérifier l'ordre de grandeur
        // DU = 0.0488 * 1_000_000 / 1000 / 183 ≈ 0.267 par jour
        assert!(du > 0.1 && du < 1.0);
    }

    #[test]
    fn test_du_zero_members() {
        let calculator = DUCalculator::new(DUConfig::default());
        let du = calculator.calculate_current_du(1_000_000.0, 0);
        assert_eq!(du, 0.0);
    }

    #[test]
    fn test_reevaluation_check() {
        let config = DUConfig::default();
        let calculator = DUCalculator::new(config);
        
        let last_reeval = Utc::now().timestamp() - (100 * 86400); // Il y a 100 jours
        let current = Utc::now().timestamp();
        
        // Ne devrait pas nécessiter de réévaluation (183 jours requis)
        assert!(!calculator.should_reevaluate(last_reeval, current));
        
        // Après 183 jours, devrait nécessiter réévaluation
        let old_reeval = Utc::now().timestamp() - (200 * 86400);
        assert!(calculator.should_reevaluate(old_reeval, current));
    }

    #[test]
    fn test_semester_du_total() {
        let calculator = DUCalculator::new(DUConfig::default());
        
        let current_mass = 1_000_000.0;
        let member_count = 1000;
        
        let semester_total = calculator.semester_du_total(current_mass, member_count);
        
        // Total semestriel = 0.0488 * 1_000_000 / 1000 = 48.8 Mahala
        assert!((semester_total - 48.8).abs() < 0.1);
    }
}

