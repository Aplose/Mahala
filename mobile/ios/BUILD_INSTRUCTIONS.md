# Instructions de compilation iOS

## Prérequis

**IMPORTANT** : Vous devez avoir un Mac avec macOS et Xcode installé.

### 1. Installer les outils

```bash
# Installer Rust (si pas déjà fait)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Installer les targets iOS
rustup target add aarch64-apple-ios        # iPhone/iPad physiques
rustup target add aarch64-apple-ios-sim    # Simulateur Apple Silicon
rustup target add x86_64-apple-ios        # Simulateur Intel (si nécessaire)
```

### 2. Compiler la bibliothèque Rust

```bash
cd ../../ffi

# Compiler pour appareils physiques
cargo build --target aarch64-apple-ios --release

# Compiler pour simulateur (Apple Silicon)
cargo build --target aarch64-apple-ios-sim --release

# Compiler pour simulateur (Intel) - optionnel
cargo build --target x86_64-apple-ios --release
```

### 3. Créer le XCFramework

```bash
# Créer le framework universel
xcodebuild -create-xcframework \
  -library target/aarch64-apple-ios/release/libmahala.a \
  -library target/aarch64-apple-ios-sim/release/libmahala.a \
  -output ../mobile/ios/Mahala/Frameworks/Mahala.xcframework
```

### 4. Créer le projet Xcode

1. Ouvrir Xcode
2. File > New > Project
3. Choisir "iOS" > "App"
4. Remplir :
   - Product Name: `Mahala`
   - Interface: `SwiftUI`
   - Language: `Swift`
   - Storage: `None` (ou Core Data si nécessaire)
5. Choisir un emplacement pour le projet

### 5. Ajouter les fichiers

1. Copier tous les fichiers de `mobile/ios/Mahala/` dans le projet Xcode
2. S'assurer que les fichiers sont ajoutés au target "Mahala"

### 6. Ajouter le framework Rust

1. Dans Xcode, sélectionner le projet dans le navigateur
2. Aller à "General" > "Frameworks, Libraries, and Embedded Content"
3. Cliquer sur "+"
4. Cliquer sur "Add Other..." > "Add Files..."
5. Sélectionner `Mahala.xcframework`
6. S'assurer que "Embed & Sign" est sélectionné

### 7. Configurer les Build Settings

1. Aller à "Build Settings"
2. Chercher "Other Linker Flags"
3. Ajouter `-lc++` si nécessaire

### 8. Configurer les permissions

1. Ouvrir `Info.plist`
2. Vérifier que `NSFaceIDUsageDescription` est présent
3. Vérifier que `UIBackgroundModes` contient `processing` et `fetch`

### 9. Tester sur simulateur

1. Choisir un simulateur iOS dans Xcode
2. Appuyer sur Run (⌘R)
3. L'application devrait se lancer

### 10. Tester sur appareil physique

1. Connecter un iPhone/iPad via USB
2. Dans Xcode, sélectionner l'appareil
3. Vous devrez peut-être configurer un compte développeur Apple
4. Appuyer sur Run

## Alternatives sans Mac

Si vous n'avez pas de Mac :

1. **GitHub Actions** : Utiliser des runners macOS pour CI/CD
2. **MacStadium** : Service cloud avec Macs
3. **AWS Mac instances** : Instances EC2 macOS
4. **Développer uniquement Android** pour l'instant

## Dépannage

### Erreur : "No such module 'Mahala'"
- Vérifier que le XCFramework est bien ajouté
- Nettoyer le build (⌘⇧K) et reconstruire

### Erreur : "Undefined symbols"
- Vérifier que toutes les bibliothèques sont liées
- Vérifier les "Other Linker Flags"

### Erreur : "Code signing"
- Configurer un compte développeur Apple
- Ou utiliser un compte gratuit pour tester sur votre propre appareil

