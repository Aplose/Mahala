//! API REST et WebSocket pour le nœud complet

pub mod rest;
pub mod websocket;

pub use rest::create_rest_app;

