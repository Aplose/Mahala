//! Dividende Universel (DU) conforme à la TRM
//!
//! Théorie Relative de la Monnaie (TRM)
//! - c = 4.88% par semestre
//! - Réévaluation tous les 183 jours
//! - Distribution quotidienne

pub mod calculator;

pub use calculator::{DUCalculator, DUConfig};

