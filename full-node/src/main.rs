//! Nœud complet Mahala
//!
//! Point d'entrée du nœud complet de la blockchain Mahala

mod config;
mod node;
mod mempool;
mod api;

use mahala_blockchain::crypto::keys::KeyPair;
use node::FullNode;
use config::NodeConfig;
use api::rest::create_rest_app;
use actix_web::middleware::Logger;
use actix_cors::Cors;
use std::sync::Arc;
use tokio::sync::RwLock;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialiser le logging
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    // Charger la configuration
    let config = NodeConfig::from_file("config.toml")
        .unwrap_or_else(|_| {
            eprintln!("Warning: Could not load config.toml, using defaults");
            NodeConfig::default()
        });
    
    // Générer ou charger la clé du validateur
    let keypair = KeyPair::new();
    let validator_key = *keypair.public_key();
    
    println!("Validator public key: {}", hex::encode(validator_key));
    
    // Créer le nœud
    let node = FullNode::new(config.clone(), validator_key);
    
    // Démarrer le nœud
    if let Err(e) = node.start().await {
        eprintln!("Error starting node: {}", e);
        std::process::exit(1);
    }
    
    // Obtenir les références pour l'API
    let blockchain = node.blockchain();
    let mempool = node.mempool();
    
    // Démarrer l'API REST
    println!("Starting API server on {}:{}", config.bind_address, config.api_port);
    
    HttpServer::new(move || {
        let app = create_rest_app(blockchain.clone(), mempool.clone());
        app()
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600)
            )
    })
    .bind(format!("{}:{}", config.bind_address, config.api_port))?
    .run()
    .await
}
