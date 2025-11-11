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

### Documents de référence

- **[MAHALA_PROJECT_PLAN.md](MAHALA_PROJECT_PLAN.md)** : Plan de développement complet avec architecture, roadmap et spécifications techniques
- **[MAHALA_METAVERSE_LUANTI_DOLIBARR.md](MAHALA_METAVERSE_LUANTI_DOLIBARR.md)** : Intégration métaverse Luanti et Dolibarr ERP, commerce virtuel et réel
- **[PROJECT_STATUS.md](PROJECT_STATUS.md)** : État actuel du projet et composants implémentés
- **[docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)** : Architecture détaillée du système
- **[docs/API.md](docs/API.md)** : Documentation complète des APIs REST
- **[docs/DEPLOYMENT.md](docs/DEPLOYMENT.md)** : Guide de déploiement et configuration

### Documentation par composant

- **[blockchain/README.md](blockchain/README.md)** : Documentation de la blockchain core
- **[light-client/README.md](light-client/README.md)** : Documentation du client léger
- **[full-node/README.md](full-node/README.md)** : Documentation du nœud complet
- **[bridge/README.md](bridge/README.md)** : Documentation du bridge Mahala ↔ June
- **[mobile/android/README.md](mobile/android/README.md)** : Documentation Android
- **[mobile/ios/README.md](mobile/ios/README.md)** : Documentation iOS
- **[luanti-mods/README.md](luanti-mods/README.md)** : Documentation des mods Luanti/Minetest
- **[dolibarr-module/README.md](dolibarr-module/README.md)** : Documentation du module Dolibarr

## 🧪 Tests

```bash
# Tous les tests
cargo test --all

# Tests blockchain uniquement
cargo test --package mahala-blockchain
```

## 📄 Licence

Ce projet est sous licence **GNU General Public License v3.0** (GPL v3).

Vous pouvez consulter le texte complet de la licence dans le fichier [LICENSE](LICENSE) ou sur le site officiel de la Free Software Foundation : [https://www.gnu.org/licenses/gpl-3.0.html](https://www.gnu.org/licenses/gpl-3.0.html)

### Résumé de la licence GPL v3

La GPL v3 est une licence copyleft qui garantit :
- ✅ **Liberté d'utiliser** : Vous pouvez utiliser le logiciel pour tout usage
- ✅ **Liberté d'étudier** : Vous avez accès au code source
- ✅ **Liberté de modifier** : Vous pouvez modifier le code
- ✅ **Liberté de distribuer** : Vous pouvez partager le logiciel et vos modifications

**Condition principale** : Si vous distribuez des versions modifiées, vous devez les distribuer sous la même licence GPL v3.

## 🤝 Contribution

Les contributions sont les bienvenues ! Veuillez ouvrir une issue ou une pull request.

### Comment contribuer

1. Fork le projet
2. Créer une branche pour votre fonctionnalité (`git checkout -b feature/AmazingFeature`)
3. Commit vos changements (`git commit -m 'Add some AmazingFeature'`)
4. Push vers la branche (`git push origin feature/AmazingFeature`)
5. Ouvrir une Pull Request

### Code de conduite

- Respecter les autres contributeurs
- Accepter les critiques constructives
- Maintenir un environnement accueillant et inclusif

## 🔗 Liens utiles

- **Site web** : [https://mahala.org](https://mahala.org) (à venir)
- **Documentation** : Voir la section [Documentation](#-documentation) ci-dessus
- **Issues** : [GitHub Issues](https://github.com/mahala/mahala/issues) (à venir)
- **Discussions** : [GitHub Discussions](https://github.com/mahala/mahala/discussions) (à venir)

## 📞 Contact

Pour toute question ou suggestion :
- Email : team@mahala.org
- Issues GitHub : Ouvrir une issue sur le dépôt

---

**Mahala** - Crypto-monnaie libre à Dividende Universel conforme à la TRM

