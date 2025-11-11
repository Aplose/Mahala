//! Light Client Mahala
//!
//! Client léger pour mobile qui synchronise avec la blockchain
//! et participe au consensus RVS en arrière-plan

pub mod sync;
pub mod validator;
pub mod storage;
pub mod p2p;

pub use sync::SyncManager;
pub use validator::ValidatorParticipant;

use mahala_blockchain::{Blockchain, PublicKey, Amount};
use mahala_blockchain::wallet::Wallet;
use mahala_blockchain::du::DUConfig;
use mahala_blockchain::consensus::rvs::RVSConfig;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Client léger Mahala pour mobile
pub struct LightClient {
    /// Blockchain locale (synchronisée)
    blockchain: Arc<RwLock<Blockchain>>,
    
    /// Wallet de l'utilisateur
    wallet: Arc<RwLock<Option<Wallet>>>,
    
    /// Participant au consensus
    validator: Arc<RwLock<ValidatorParticipant>>,
    
    /// Configuration
    config: LightClientConfig,
}

/// Configuration du light client
#[derive(Debug, Clone)]
pub struct LightClientConfig {
    /// URL du nœud complet pour synchronisation
    pub full_node_url: String,
    
    /// Intervalle de synchronisation (secondes)
    pub sync_interval: u64,
    
    /// Activer la participation au consensus
    pub enable_consensus: bool,
}

impl Default for LightClientConfig {
    fn default() -> Self {
        Self {
            full_node_url: "http://localhost:8080".to_string(),
            sync_interval: 10, // 10 secondes
            enable_consensus: true,
        }
    }
}

impl LightClient {
    /// Créer un nouveau light client
    pub fn new(config: LightClientConfig) -> Self {
        let du_config = DUConfig::default();
        let rvs_config = RVSConfig::default();
        let blockchain = Arc::new(RwLock::new(Blockchain::new(du_config, rvs_config)));
        
        let validator = Arc::new(RwLock::new(ValidatorParticipant::new()));
        
        Self {
            blockchain,
            wallet: Arc::new(RwLock::new(None)),
            validator,
            config,
        }
    }

    /// Créer un wallet depuis un hash biométrique
    pub async fn create_wallet_from_biometric(&self, biometric_hash: &[u8; 32]) -> Result<PublicKey, String> {
        // Utiliser le hash biométrique comme seed pour générer le wallet de manière déterministe
        let wallet = Wallet::from_seed(biometric_hash);
        let address = *wallet.address();
        
        // Enregistrer le wallet
        let mut wallet_guard = self.wallet.write().await;
        *wallet_guard = Some(wallet);
        
        // Enregistrer comme validateur si activé
        if self.config.enable_consensus {
            let mut blockchain_guard = self.blockchain.write().await;
            blockchain_guard.consensus_mut().register_validator(address, address);
        }
        
        Ok(address)
    }

    /// Obtenir la balance du wallet
    pub async fn get_balance(&self) -> Result<Amount, String> {
        let wallet_guard = self.wallet.read().await;
        let wallet = wallet_guard.as_ref()
            .ok_or("Wallet not initialized")?;
        
        let blockchain_guard = self.blockchain.read().await;
        Ok(blockchain_guard.get_balance(wallet.address()))
    }

    /// Synchroniser avec le réseau
    pub async fn sync(&self) -> Result<(), String> {
        // TODO: Implémenter la synchronisation avec les nœuds complets
        // Pour l'instant, on simule
        Ok(())
    }

    /// Participer au consensus (appelé périodiquement en arrière-plan)
    pub async fn participate_consensus(&self) -> Result<bool, String> {
        let blockchain_guard = self.blockchain.read().await;
        let wallet_guard = self.wallet.read().await;
        
        let wallet = wallet_guard.as_ref()
            .ok_or("Wallet not initialized")?;
        
        let last_hash = blockchain_guard.last_block_hash()
            .ok_or("No blocks in chain")?;
        
        let selection = blockchain_guard.consensus().select_validators(last_hash);
        
        // Vérifier si ce wallet est sélectionné
        let is_selected = blockchain_guard.consensus()
            .is_validator_selected(wallet.address(), &selection);
        
        Ok(is_selected)
    }

    /// Obtenir le Dividende Universel du jour
    pub async fn get_daily_du(&self) -> Result<Amount, String> {
        // Calculer le DU actuel
        let du_calculator = mahala_blockchain::du::DUCalculator::new(DUConfig::default());
        // Pour l'instant, on utilise des valeurs par défaut
        // Dans une vraie implémentation, on récupérerait ces valeurs depuis la blockchain
        let current_mass = 1_000_000.0; // TODO: Récupérer depuis la blockchain
        let member_count = 1000; // TODO: Récupérer depuis la blockchain
        
        Ok(du_calculator.calculate_current_du(current_mass, member_count))
    }

    /// Obtenir le wallet (pour utilisation interne)
    pub async fn get_wallet(&self) -> Option<Wallet> {
        self.wallet.read().await.clone()
    }
}
