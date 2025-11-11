# Architecture Mahala

## Vue d'ensemble

Mahala est une blockchain légère optimisée pour mobile avec consensus RVS et Dividende Universel conforme à la TRM.

## Composants principaux

### 1. Blockchain Core (`blockchain/`)

Cœur de la blockchain en Rust :
- Structures de blocs et transactions
- Consensus RVS (Random Validator Selection)
- Calcul du Dividende Universel
- Smart Contracts NFT
- Cryptographie (Ed25519, Blake3)

### 2. Light Client (`light-client/`)

Client léger pour mobile :
- Synchronisation par checkpoints
- Participation au consensus
- Stockage minimal
- Optimisé pour batterie

### 3. Full Node (`full-node/`)

Nœud complet de la blockchain :
- Stockage complet de la blockchain
- API REST pour les clients
- Mempool pour transactions
- Production de blocs

### 4. Bridge (`bridge/`)

Pont entre Mahala et June (Ğ1) :
- Market Maker (AMM)
- Gestion des réserves
- API REST pour échanges
- Sécurité et limites

### 5. FFI (`ffi/`)

Interface Foreign Function pour mobile :
- Bindings C pour Android/iOS
- Wrapper async
- Gestion de la mémoire

### 6. Applications mobiles (`mobile/`)

- **Android** : Kotlin + Jetpack Compose
- **iOS** : Swift + SwiftUI
- Biométrie pour création wallet
- Service en arrière-plan pour DU

### 7. Module Dolibarr (`dolibarr-module/`)

Intégration ERP :
- Gestion wallets tiers
- Échanges via bridge
- Boutiques virtuelles

## Flux de données

```
┌─────────────┐
│ Mobile App  │
│ (Light      │
│  Client)    │
└──────┬──────┘
       │
       │ API REST
       │
┌──────▼──────┐
│  Full Node  │◄─────┐
│             │      │
│  Blockchain │      │ P2P
│  + Mempool  │      │
└──────┬──────┘      │
       │             │
       │             │
┌──────▼──────┐      │
│   Bridge    │      │
│  Mahala ↔   │      │
│    June     │      │
└─────────────┘      │
                     │
              ┌──────┴──────┐
              │ Other Nodes │
              └─────────────┘
```

## Consensus RVS

1. **Sélection aléatoire** : VRF pour choisir les validateurs
2. **Quorum 67%** : Nécessite 67% des validateurs pour valider
3. **Rotation** : Nouveaux validateurs toutes les 5 secondes
4. **Participation mobile** : Les apps mobiles peuvent participer

## Dividende Universel

- **Taux** : 4.88% par semestre (c)
- **Distribution** : Quotidienne
- **Formule** : DU(t) = c * M(t) / N(t)
- **Réévaluation** : Tous les 183 jours

## Sécurité

- **Cryptographie** : Ed25519 (signatures), Blake3 (hashing)
- **Biométrie** : Hash déterministe, données jamais stockées
- **Limites** : Bridge avec limites quotidiennes/mensuelles
- **Validation** : Transactions signées et vérifiées

## Performance

- **Blocs** : 5 secondes
- **TPS** : >1000 transactions/seconde
- **Mobile** : <100MB RAM, <5% batterie/jour
- **Sync** : <10 secondes pour light client

## Évolutivité

- **Sharding** : À implémenter pour scaling
- **P2P** : Réseau décentralisé avec libp2p
- **Checkpoints** : Synchronisation rapide
- **SPV** : Vérification simplifiée

