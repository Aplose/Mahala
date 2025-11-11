//! Protocole P2P simplifié pour mobile

use std::net::SocketAddr;

/// Protocole P2P allégé
pub struct P2PProtocol {
    /// Adresses des pairs connus
    known_peers: Vec<SocketAddr>,
}

impl P2PProtocol {
    /// Créer un nouveau protocole P2P
    pub fn new() -> Self {
        Self {
            known_peers: Vec::new(),
        }
    }

    /// Ajouter un pair
    pub fn add_peer(&mut self, addr: SocketAddr) {
        if !self.known_peers.contains(&addr) {
            self.known_peers.push(addr);
        }
    }

    /// Obtenir les pairs connus
    pub fn known_peers(&self) -> &[SocketAddr] {
        &self.known_peers
    }
}

impl Default for P2PProtocol {
    fn default() -> Self {
        Self::new()
    }
}

