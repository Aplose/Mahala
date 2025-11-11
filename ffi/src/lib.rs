//! FFI (Foreign Function Interface) pour Mahala
//!
//! Interface simple pour les applications mobiles Android/iOS
//! Utilise des fonctions C simples pour l'interopérabilité

use mahala_light_client::{LightClient, LightClientConfig};
use mahala_blockchain::PublicKey;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Instance globale du light client (singleton)
static mut CLIENT: Option<Arc<RwLock<LightClient>>> = None;
static INIT: std::sync::Once = std::sync::Once::new();

/// Initialiser le light client
#[no_mangle]
pub extern "C" fn init_light_client(full_node_url: *const c_char) -> i32 {
    let url = unsafe {
        match CStr::from_ptr(full_node_url).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return -1,
        }
    };

    INIT.call_once(|| {
        let config = LightClientConfig {
            full_node_url: url,
            sync_interval: 10,
            enable_consensus: true,
        };
        let client = LightClient::new(config);
        unsafe {
            CLIENT = Some(Arc::new(RwLock::new(client)));
        }
    });
    0
}

/// Obtenir l'instance du client
fn get_client() -> Result<Arc<RwLock<LightClient>>, String> {
    unsafe {
        Ok(CLIENT.as_ref()
            .ok_or("Light client not initialized. Call init_light_client first.")?
            .clone())
    }
}

/// Créer un wallet depuis un hash biométrique
/// Retourne l'adresse en hexadécimal dans le buffer fourni
#[no_mangle]
pub extern "C" fn create_wallet_from_biometric(
    biometric_hash_hex: *const c_char,
    output: *mut c_char,
    output_len: usize,
) -> i32 {
    let hash_hex = unsafe {
        match CStr::from_ptr(biometric_hash_hex).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return -1,
        }
    };

    // Décoder le hash hexadécimal
    let hash_bytes = match hex::decode(&hash_hex) {
        Ok(bytes) => bytes,
        Err(_) => return -2,
    };

    if hash_bytes.len() != 32 {
        return -3;
    }

    let mut hash = [0u8; 32];
    hash.copy_from_slice(&hash_bytes);

    // Utiliser tokio runtime pour les appels async
    let rt = tokio::runtime::Runtime::new().unwrap();
    let client = match get_client() {
        Ok(c) => c,
        Err(_) => return -4,
    };

    let address = rt.block_on(async {
        let client_guard = client.read().await;
        client_guard.create_wallet_from_biometric(&hash).await
    });

    match address {
        Ok(addr) => {
            let addr_hex = hex::encode(addr);
            let addr_cstr = CString::new(addr_hex).unwrap();
            let addr_bytes = addr_cstr.as_bytes_with_nul();
            
            if addr_bytes.len() > output_len {
                return -5;
            }

            unsafe {
                std::ptr::copy_nonoverlapping(
                    addr_bytes.as_ptr(),
                    output as *mut u8,
                    addr_bytes.len(),
                );
            }
            0
        }
        Err(_) => -6,
    }
}

/// Obtenir la balance
#[no_mangle]
pub extern "C" fn get_balance() -> f64 {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let client = match get_client() {
        Ok(c) => c,
        Err(_) => return 0.0,
    };

    rt.block_on(async {
        let client_guard = client.read().await;
        client_guard.get_balance().await.unwrap_or(0.0)
    })
}

/// Obtenir le Dividende Universel du jour
#[no_mangle]
pub extern "C" fn get_daily_du() -> f64 {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let client = match get_client() {
        Ok(c) => c,
        Err(_) => return 0.0,
    };

    rt.block_on(async {
        let client_guard = client.read().await;
        client_guard.get_daily_du().await.unwrap_or(0.0)
    })
}

/// Vérifier si le wallet est sélectionné comme validateur
#[no_mangle]
pub extern "C" fn check_if_selected_validator() -> i32 {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let client = match get_client() {
        Ok(c) => c,
        Err(_) => return 0,
    };

    let result = rt.block_on(async {
        let client_guard = client.read().await;
        client_guard.participate_consensus().await
    });

    match result {
        Ok(true) => 1,
        _ => 0,
    }
}

/// Synchroniser avec le réseau
#[no_mangle]
pub extern "C" fn sync() -> i32 {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let client = match get_client() {
        Ok(c) => c,
        Err(_) => return -1,
    };

    let result = rt.block_on(async {
        let client_guard = client.read().await;
        client_guard.sync().await
    });

    match result {
        Ok(_) => 0,
        Err(_) => -1,
    }
}
