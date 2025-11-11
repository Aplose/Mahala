# Mahala - Crypto-monnaie libre à Dividende Universel

Blockchain légère optimisée pour mobile avec consensus RVS et Dividende Universel conforme à la TRM.

## 🎯 Caractéristiques

- ✅ **Blockchain légère** : Optimisée pour smartphone
- ✅ **Consensus RVS** : Random Validator Selection avec VRF
- ✅ **Dividende Universel** : Calcul conforme à la TRM (4.88% par semestre)
- ✅ **Applications mobiles** : Android et iOS avec biométrie
- ✅ **Bridge June** : Échange avec Ğ1/Duniter via AMM
- ✅ **Smart Contracts NFT** : Support des NFT pour le métaverse

## 📁 Structure du projet

```
Mahala/
├── blockchain/          # Coeur blockchain Rust
├── light-client/        # Client léger pour mobile
├── full-node/           # Nœud complet avec API REST
├── bridge/              # Bridge Mahala ↔ June
├── ffi/                 # Interface FFI pour mobile
├── mobile/              # Applications mobiles
│   ├── android/         # App Android (Kotlin)
│   └── ios/             # App iOS (Swift)
├── luanti-mods/         # Mods Minetest/Luanti
│   ├── mahala_teleport/ # Téléportation payante
│   ├── mahala_rental/   # Location d'emplacements
│   ├── mahala_nft/      # Affichage NFT
│   └── mahala_shop/     # Boutiques virtuelles
└── dolibarr-module/     # Module Dolibarr ERP
```

## 🚀 Démarrage rapide

### Compiler la blockchain

```bash
cargo build --release
```

### Lancer un nœud complet

```bash
cd full-node
cargo run --release
```

Le nœud sera accessible sur `http://localhost:8080`

### Lancer le bridge

```bash
cd bridge
cargo run --release
```

Le bridge sera accessible sur `http://localhost:8081`

## 📱 Applications mobiles

### Android

Voir `mobile/android/README.md` pour les instructions de compilation.

### iOS

**Nécessite macOS et Xcode**

Voir `mobile/ios/README.md` et `mobile/ios/BUILD_INSTRUCTIONS.md` pour les instructions.

## 🔧 Configuration

### Full Node

Éditer `full-node/config.toml` :

```toml
api_port = 8080
p2p_port = 9000
bind_address = "0.0.0.0"
```

### Bridge

Éditer `bridge/config.toml` :

```toml
port = 8081
initial_june_reserve = 10000.0
initial_mahala_reserve = 10000.0
```

## 📚 Documentation

- `MAHALA_PROJECT_PLAN.md` : Plan de développement complet
- `MAHALA_METAVERSE_LUANTI_DOLIBARR.md` : Intégration métaverse et Dolibarr
- `blockchain/README.md` : Documentation de la blockchain
- `mobile/android/README.md` : Documentation Android
- `mobile/ios/README.md` : Documentation iOS

## 🧪 Tests

```bash
# Tous les tests
cargo test --all

# Tests blockchain uniquement
cargo test --package mahala-blockchain
```

## 📄 Licence

MIT

## 🤝 Contribution

Les contributions sont les bienvenues ! Veuillez ouvrir une issue ou une pull request.

