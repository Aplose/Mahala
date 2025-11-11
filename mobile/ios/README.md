# Application iOS Mahala

Application mobile iOS pour créer un portefeuille membre Mahala avec validation biométrique et participation au consensus.

## 🍎 Prérequis

**IMPORTANT** : Pour compiler et tester l'application iOS, vous devez avoir :

- **macOS** (MacBook, iMac, Mac mini, etc.)
- **Xcode** (version 15.0 ou plus récente)
- **CocoaPods** ou **Swift Package Manager** (pour les dépendances)
- **Rust toolchain** pour compiler la bibliothèque native

### Pourquoi macOS est nécessaire ?

- Xcode ne fonctionne que sur macOS
- Les simulateurs iOS nécessitent macOS
- La compilation pour appareils physiques nécessite Xcode
- Les outils de développement Apple sont exclusifs à macOS

### Alternatives sans Mac

Si vous n'avez pas de Mac, vous pouvez :

1. **Utiliser un Mac virtuel** (contre les conditions d'utilisation d'Apple)
2. **Utiliser un service cloud** comme MacStadium ou AWS Mac instances
3. **Développer uniquement Android** pour l'instant
4. **Utiliser GitHub Actions** avec des runners macOS pour CI/CD

## 🔧 Configuration

### 1. Compiler la bibliothèque Rust pour iOS

```bash
# Installer les targets iOS
rustup target add aarch64-apple-ios        # iPhone/iPad physiques
rustup target add x86_64-apple-ios        # Simulateur Intel
rustup target add aarch64-apple-ios-sim    # Simulateur Apple Silicon

# Compiler pour iOS
cd ../../ffi
cargo build --target aarch64-apple-ios --release
cargo build --target aarch64-apple-ios-sim --release

# Créer un XCFramework
xcodebuild -create-xcframework \
  -library target/aarch64-apple-ios/release/libmahala.a \
  -library target/aarch64-apple-ios-sim/release/libmahala.a \
  -output ../mobile/ios/Mahala/Frameworks/Mahala.xcframework
```

### 2. Créer le projet Xcode

1. Ouvrir Xcode
2. Créer un nouveau projet iOS > App
3. Nom : "Mahala"
4. Interface : SwiftUI
5. Language : Swift
6. Copier les fichiers de ce répertoire dans le projet

### 3. Ajouter le framework Rust

1. Dans Xcode, aller à "General" > "Frameworks, Libraries, and Embedded Content"
2. Ajouter `Mahala.xcframework`
3. S'assurer que "Embed & Sign" est sélectionné

## 📱 Structure de l'application

```
Mahala/
├── MahalaApp.swift              # Point d'entrée
├── MahalaCore.swift             # Wrapper FFI Rust
├── Views/
│   ├── ContentView.swift        # Vue principale
│   ├── WalletView.swift         # Vue du wallet
│   └── CreateWalletView.swift   # Vue de création
├── ViewModels/
│   └── WalletViewModel.swift    # ViewModel
├── Services/
│   ├── BiometricService.swift   # Service biométrie
│   └── ValidatorService.swift   # Service arrière-plan
└── Frameworks/
    └── Mahala.xcframework       # Bibliothèque Rust
```

## 🚀 Fonctionnalités

- ✅ Création de wallet avec Face ID / Touch ID
- ✅ Nœud léger synchronisé
- ✅ Service en arrière-plan pour consensus et DU
- ✅ Interface SwiftUI moderne

## 🔒 Permissions

Dans `Info.plist`, ajouter :

```xml
<key>NSFaceIDUsageDescription</key>
<string>Mahala utilise Face ID pour créer votre wallet de manière sécurisée</string>
```

## 📝 Notes

- Le service en arrière-plan utilise `BGTaskScheduler` pour iOS
- Les notifications nécessitent une autorisation utilisateur
- L'app doit être ajoutée aux exceptions d'optimisation batterie

## 🐛 Débogage

```bash
# Voir les logs
xcrun simctl spawn booted log stream --predicate 'process == "Mahala"'

# Tester sur un appareil physique
# Nécessite un compte développeur Apple
```

## 📄 Licence

MIT

