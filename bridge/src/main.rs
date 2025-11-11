//! Bridge Mahala ↔ June
//!
//! Service de pont entre Mahala et June (Ğ1/Duniter)
//! avec un Automated Market Maker (AMM)

mod market_maker;
mod reserves;
mod june_client;
mod security;

use actix_web::{web, App, HttpServer, HttpResponse, Result as ActixResult};
use actix_web::middleware::Logger;
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

use market_maker::{MarketMaker, ExchangeDirection, ExchangeRequest, ExchangeResult};
use reserves::Reserves;
use security::SecurityManager;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialiser le logging
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    // Initialiser les réserves (10k June, 10k Mahala)
    let reserves = Arc::new(RwLock::new(Reserves::new(10_000.0, 10_000.0)));
    
    // Créer le market maker
    let market_maker = Arc::new(RwLock::new(MarketMaker::new(10_000.0, 10_000.0)));
    
    // Créer le gestionnaire de sécurité
    let security = Arc::new(SecurityManager::new(1000.0, 5000.0)); // 1k/jour, 5k/mois
    
    println!("Starting Mahala Bridge on 0.0.0.0:8081");
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(reserves.clone()))
            .app_data(web::Data::new(market_maker.clone()))
            .app_data(web::Data::new(security.clone()))
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600)
            )
            .route("/health", web::get().to(health))
            .route("/bridge/stats", web::get().to(get_stats))
            .route("/bridge/quote", web::post().to(get_quote))
            .route("/bridge/exchange", web::post().to(execute_exchange))
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}

/// Health check
async fn health() -> ActixResult<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "mahala-bridge"
    })))
}

/// Obtenir les statistiques du bridge
async fn get_stats(
    reserves: web::Data<Arc<RwLock<Reserves>>>,
    market_maker: web::Data<Arc<RwLock<MarketMaker>>>,
) -> ActixResult<HttpResponse> {
    let reserve_stats = reserves.read().await.get_stats().await;
    let mm_stats = market_maker.read().await.get_stats();
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "reserves": {
            "june": reserve_stats.june,
            "mahala": reserve_stats.mahala,
        },
        "market_maker": {
            "june_reserve": mm_stats.june_reserve,
            "mahala_reserve": mm_stats.mahala_reserve,
            "total_liquidity": mm_stats.total_liquidity,
            "fee_percentage": mm_stats.fee_percentage,
        }
    })))
}

/// Obtenir un devis
#[derive(Deserialize)]
struct QuoteRequest {
    direction: String, // "mahala_to_june" ou "june_to_mahala"
    amount: f64,
}

async fn get_quote(
    req: web::Json<QuoteRequest>,
    market_maker: web::Data<Arc<RwLock<MarketMaker>>>,
) -> ActixResult<HttpResponse> {
    let direction = match req.direction.as_str() {
        "mahala_to_june" => ExchangeDirection::MahalaToJune,
        "june_to_mahala" => ExchangeDirection::JuneToMahala,
        _ => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid direction"
            })));
        }
    };

    let mm = market_maker.read().await;
    let quote = match direction {
        ExchangeDirection::MahalaToJune => mm.quote_mahala_to_june(req.amount),
        ExchangeDirection::JuneToMahala => mm.quote_june_to_mahala(req.amount),
    };

    match quote {
        Ok(q) => Ok(HttpResponse::Ok().json(q)),
        Err(e) => Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Exécuter un échange
async fn execute_exchange(
    req: web::Json<ExchangeRequest>,
    market_maker: web::Data<Arc<RwLock<MarketMaker>>>,
    security: web::Data<Arc<SecurityManager>>,
) -> ActixResult<HttpResponse> {
    // Vérifier les limites de sécurité
    if let Err(e) = security.check_exchange(&req.wallet_address, req.amount).await {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": e.to_string()
        })));
    }

    // Exécuter l'échange
    let mut mm = market_maker.write().await;
    match mm.execute_exchange(req.direction, req.amount) {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(e) => Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

