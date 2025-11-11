//! API REST pour le nœud complet

use actix_web::{web, App, HttpServer, HttpResponse, Result as ActixResult};
use mahala_blockchain::{Blockchain, PublicKey};
use mahala_blockchain::transaction::Transaction;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Créer l'application REST
pub fn create_rest_app(
    blockchain: Arc<RwLock<Blockchain>>,
    mempool: Arc<crate::mempool::Mempool>,
) -> impl Fn() -> App<impl actix_web::dev::ServiceFactory<actix_web::dev::ServiceRequest, Config = (), Error = actix_web::Error, InitError = ()>> {
    move || {
        App::new()
            .app_data(web::Data::new(blockchain.clone()))
            .app_data(web::Data::new(mempool.clone()))
            .route("/health", web::get().to(health))
            .route("/blockchain/height", web::get().to(get_height))
            .route("/blockchain/last_block", web::get().to(get_last_block))
            .route("/blockchain/block/{height}", web::get().to(get_block))
            .route("/blockchain/balance/{address}", web::get().to(get_balance))
            .route("/transaction/submit", web::post().to(submit_transaction))
            .route("/mempool/size", web::get().to(get_mempool_size))
    }
}

/// Health check
async fn health() -> ActixResult<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "mahala-full-node"
    })))
}

/// Obtenir la hauteur de la blockchain
async fn get_height(
    blockchain: web::Data<Arc<RwLock<Blockchain>>>,
) -> ActixResult<HttpResponse> {
    let height = blockchain.read().await.height();
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "height": height
    })))
}

/// Obtenir le dernier bloc
async fn get_last_block(
    blockchain: web::Data<Arc<RwLock<Blockchain>>>,
) -> ActixResult<HttpResponse> {
    let blockchain_guard = blockchain.read().await;
    let height = blockchain_guard.height();
    
    if height == 0 {
        return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "No blocks"
        })));
    }
    
    if let Some(block) = blockchain_guard.get_block(height - 1) {
        Ok(HttpResponse::Ok().json(block))
    } else {
        Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Block not found"
        })))
    }
}

/// Obtenir un bloc par hauteur
async fn get_block(
    path: web::Path<u64>,
    blockchain: web::Data<Arc<RwLock<Blockchain>>>,
) -> ActixResult<HttpResponse> {
    let height = path.into_inner();
    let blockchain_guard = blockchain.read().await;
    
    if let Some(block) = blockchain_guard.get_block(height) {
        Ok(HttpResponse::Ok().json(block))
    } else {
        Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Block not found"
        })))
    }
}

/// Obtenir la balance d'une adresse
async fn get_balance(
    path: web::Path<String>,
    blockchain: web::Data<Arc<RwLock<Blockchain>>>,
) -> ActixResult<HttpResponse> {
    let address_hex = path.into_inner();
    
    // Décoder l'adresse hex
    let address_bytes = match hex::decode(&address_hex) {
        Ok(bytes) if bytes.len() == 32 => {
            let mut addr = [0u8; 32];
            addr.copy_from_slice(&bytes);
            addr
        }
        _ => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid address format"
            })));
        }
    };
    
    let blockchain_guard = blockchain.read().await;
    let balance = blockchain_guard.get_balance(&address_bytes);
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "address": address_hex,
        "balance": balance
    })))
}

/// Soumettre une transaction
#[derive(Deserialize)]
struct SubmitTransactionRequest {
    transaction: Transaction,
}

async fn submit_transaction(
    req: web::Json<SubmitTransactionRequest>,
    mempool: web::Data<Arc<crate::mempool::Mempool>>,
) -> ActixResult<HttpResponse> {
    let tx = req.into_inner().transaction;
    
    match mempool.add_transaction(tx.clone()).await {
        Ok(_) => {
            let mut tx_copy = tx.clone();
            let hash = tx_copy.calculate_hash();
            let hash_hex = hex::encode(hash);
            
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "status": "accepted",
                "tx_hash": hash_hex
            })))
        }
        Err(e) => {
            Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": e.to_string()
            })))
        }
    }
}

/// Obtenir la taille du mempool
async fn get_mempool_size(
    mempool: web::Data<Arc<crate::mempool::Mempool>>,
) -> ActixResult<HttpResponse> {
    let size = mempool.size().await;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "size": size
    })))
}

