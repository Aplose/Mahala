//! Configuration du nœud complet

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuration du nœud
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    /// Port de l'API REST
    pub api_port: u16,
    
    /// Port du réseau P2P
    pub p2p_port: u16,
    
    /// Adresse d'écoute
    pub bind_address: String,
    
    /// Répertoire de données
    pub data_dir: PathBuf,
    
    /// Bootstrap nodes (peers initiaux)
    pub bootstrap_nodes: Vec<String>,
    
    /// Activer le mode debug
    pub debug: bool,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            api_port: 8080,
            p2p_port: 9000,
            bind_address: "0.0.0.0".to_string(),
            data_dir: PathBuf::from("./data"),
            bootstrap_nodes: Vec::new(),
            debug: false,
        }
    }
}

impl NodeConfig {
    /// Charger depuis un fichier TOML
    pub fn from_file(path: &str) -> Result<Self, config::ConfigError> {
        let settings = config::Config::builder()
            .add_source(config::File::with_name(path))
            .add_source(config::Environment::with_prefix("MAHALA"))
            .build()?;
        
        settings.try_deserialize()
    }
}

