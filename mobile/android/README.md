# Application Android Mahala

Application mobile Android pour créer un portefeuille membre Mahala avec validation biométrique et participation au consensus.

## 🎯 Fonctionnalités

- ✅ **Création de wallet avec biométrie** : Utilise la biométrie pour générer un wallet de manière sécurisée et déterministe
- ✅ **Nœud léger** : Synchronise avec la blockchain sans télécharger tous les blocs
- ✅ **Service en arrière-plan** : Tourne en continu pour participer au consensus et recevoir le DU
- ✅ **Interface moderne** : Jetpack Compose avec Material 3

## 📋 Prérequis

- Android Studio Hedgehog ou plus récent
- Android SDK 26+ (Android 8.0+)
- Rust toolchain pour compiler la bibliothèque native
- NDK pour Android

## 🔧 Configuration

### 1. Compiler la bibliothèque Rust

```bash
# Installer les targets Android
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add x86_64-linux-android

# Compiler pour Android
cd ../../ffi
cargo build --target aarch64-linux-android --release

# Copier la bibliothèque dans le projet Android
cp target/aarch64-linux-android/release/libmahala.so \
   ../mobile/android/app/src/main/jniLibs/arm64-v8a/
```

### 2. Configurer Gradle

Le fichier `build.gradle.kts` est déjà configuré. Assurez-vous que :
- `minSdk = 26`
- Les bibliothèques natives sont dans `app/src/main/jniLibs/`

### 3. Permissions

Les permissions nécessaires sont déjà déclarées dans `AndroidManifest.xml` :
- Internet
- Réseau
- Service en avant-plan
- Biométrie

## 🚀 Utilisation

### Créer un wallet

1. Lancer l'application
2. Appuyer sur "Créer avec biométrie"
3. Authentifier avec Face ID / Touch ID / PIN
4. Le wallet est créé automatiquement

### Recevoir le Dividende Universel

- **Important** : L'application doit tourner en arrière-plan pour recevoir le DU
- Le service `ValidatorService` démarre automatiquement
- La synchronisation se fait toutes les 15 minutes

### Participer au consensus

- Quand le wallet est sélectionné comme validateur, une notification apparaît
- Le service signe automatiquement les blocs si possible

## 📱 Structure

```
app/src/main/java/com/mahala/
├── MainActivity.kt              # Activité principale
├── MahalaApplication.kt         # Application (démarre le service)
├── MahalaCore.kt                # Wrapper FFI Rust
├── biometric/
│   └── BiometricManager.kt      # Gestion biométrie
├── wallet/
│   ├── WalletViewModel.kt       # ViewModel
│   └── WalletScreen.kt          # UI Compose
└── validator/
    └── ValidatorService.kt       # Service arrière-plan
```

## 🔒 Sécurité

- **Biométrie** : Les données biométriques ne sont jamais stockées
- **Hash déterministe** : Le hash est généré à partir de l'Android ID + salt
- **Stockage sécurisé** : Les clés privées sont stockées de manière sécurisée (à implémenter avec Android Keystore)

## 📝 Notes

- Le service en arrière-plan consomme de la batterie mais est nécessaire pour le DU
- Android peut tuer le service si la batterie est faible (utiliser `START_STICKY`)
- Pour une meilleure expérience, ajouter l'app à la liste des exceptions d'optimisation batterie

## 🐛 Débogage

```bash
# Voir les logs
adb logcat | grep Mahala

# Vérifier que le service tourne
adb shell dumpsys activity services | grep ValidatorService
```

## 📄 Licence

MIT

