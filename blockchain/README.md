# Mahala Blockchain Core

Implémentation de la blockchain Mahala en Rust, optimisée pour mobile avec consensus RVS et Dividende Universel conforme à la TRM.

## 🎯 Caractéristiques

- **Blockchain légère** : Optimisée pour fonctionner sur smartphone
- **Consensus RVS** : Random Validator Selection avec VRF pour sélection aléatoire vérifiable
- **Dividende Universel** : Calcul conforme à la Théorie Relative de la Monnaie (TRM)
- **Cryptographie moderne** : Ed25519 pour signatures, Blake3 pour hashing
- **Smart Contracts NFT** : Support des NFT pour le métaverse
- **Stockage optimisé** : Checkpoints et arbres de Merkle pour synchronisation rapide

## 📁 Structure

```
blockchain/
├── src/
│   ├── lib.rs              # Point d'entrée principal
│   ├── block.rs            # Structures de blocs
│   ├── chain.rs            # Gestion de la chaîne
│   ├── transaction.rs      # Transactions
│   ├── wallet.rs           # Gestion des wallets
│   ├── consensus/          # Consensus RVS
│   │   ├── mod.rs
│   │   ├── rvs.rs          # Random Validator Selection
│   │   └── vrf.rs          # Verifiable Random Function
│   ├── crypto/             # Cryptographie
│   │   ├── mod.rs
│   │   ├── keys.rs         # Gestion des clés
│   │   ├── signatures.rs   # Signatures
│   │   └── hash.rs         # Hashing
│   ├── du/                 # Dividende Universel
│   │   ├── mod.rs
│   │   └── calculator.rs  # Calcul DU selon TRM
│   ├── storage/            # Stockage léger
│   │   ├── mod.rs
│   │   ├── merkle.rs       # Arbres de Merkle
│   │   └── checkpoint.rs   # Points de contrôle
│   └── nft/                # Smart Contracts NFT
│       ├── mod.rs
│       └── contract.rs     # Contrat NFT
```

## 🚀 Utilisation

### Créer une blockchain

```rust
use mahala_blockchain::*;
use mahala_blockchain::du::DUConfig;
use mahala_blockchain::consensus::rvs::RVSConfig;

let du_config = DUConfig::default();
let rvs_config = RVSConfig::default();
let mut blockchain = Blockchain::new(du_config, rvs_config);

// Créer le bloc genesis
let keypair = KeyPair::new();
blockchain.create_genesis(*keypair.public_key())?;
```

### Créer un wallet

```rust
use mahala_blockchain::wallet::Wallet;

let wallet = Wallet::new();
let address = wallet.address();
```

### Créer une transaction

```rust
use mahala_blockchain::transaction::*;

let mut tx = Transaction::new(
    sender_public_key,
    receiver_public_key,
    100.0,  // montant
    0.1,    // frais
    TransactionMetadata::default(),
);

tx.sign(sender_private_key)?;
assert!(tx.is_valid());
```

### Consensus RVS

```rust
use mahala_blockchain::consensus::rvs::RVS;

let mut rvs = RVS::new(RVSConfig::default());
rvs.register_validator(validator_public_key, wallet_address);

let previous_hash = blockchain.last_block_hash().unwrap();
let selection = rvs.select_validators(previous_hash);
```

### Calcul DU

```rust
use mahala_blockchain::du::DUCalculator;

let calculator = DUCalculator::new(DUConfig::default());
let du = calculator.calculate_current_du(
    current_mass,    // Masse monétaire actuelle
    member_count,   // Nombre de membres
);
```

## 🔧 Configuration

### DU (Dividende Universel)

- **Taux de croissance** : 4.88% par semestre (c)
- **Durée semestre** : 183 jours
- **Distribution** : Quotidienne

### Consensus RVS

- **Intervalle blocs** : 5 secondes
- **Nombre validateurs** : 10 par bloc
- **Quorum requis** : 67%

## 🧪 Tests

```bash
cargo test --package mahala-blockchain
```

## 📦 Dépendances

- `ed25519-dalek` : Signatures cryptographiques
- `blake3` : Hashing rapide
- `serde` : Sérialisation
- `chrono` : Gestion du temps
- `thiserror` : Gestion d'erreurs

## 🎯 Optimisations Mobile

- **Hashing rapide** : Blake3 au lieu de SHA-256
- **Signatures compactes** : Ed25519 (64 bytes)
- **Stockage minimal** : Checkpoints tous les 100 blocs
- **SPV** : Vérification simplifiée avec preuves Merkle

## 📝 Notes

Cette implémentation est une version initiale optimisée pour la performance mobile. Pour la production, considérer :

- Implémentation VRF complète (ECVRF)
- Persistance avec RocksDB ou SQLite
- Réseau P2P avec libp2p
- Tests de charge et optimisation

## 📄 Licence

MIT

