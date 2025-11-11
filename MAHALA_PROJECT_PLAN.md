# Plan de développement Mahala - Cursor IDE

## 📋 Vue d'ensemble du projet

**Projet**: Mahala - Crypto-monnaie libre à Dividende Universel  
**Stack principal**: Rust (blockchain) + Native mobile (Kotlin/Swift)  
**Objectif**: Monnaie libre accessible avec consensus léger sur smartphone

### Caractéristiques clés
- ✅ Dividende Universel conforme TRM
- ✅ 1 humain = 1 wallet (biométrie multi-facteurs)
- ✅ Consensus par élection aléatoire (RVS - Random Validator Selection)
- ✅ Blockchain ultra-légère pour smartphone
- ✅ Bridge avec June (Ğ1/Duniter)
- ✅ Intégration Dolibarr ERP

---

## 🏗️ Architecture globale

```
┌─────────────────────────────────────────┐
│   Apps Natives                          │
│   ├─ Android (Kotlin)                   │
│   └─ iOS (Swift)                        │
│                                         │
│   FFI (Foreign Function Interface)      │
│   ↓                                     │
├─────────────────────────────────────────┤
│   Mahala Core (Rust)                    │
│   ├─ Light Client P2P                   │
│   ├─ Wallet & Crypto                    │
│   ├─ Consensus Participant              │
│   └─ Bridge Client                      │
├─────────────────────────────────────────┤
│   Backend Services (Rust)               │
│   ├─ Full Node                          │
│   ├─ Bridge Service                     │
│   └─ API REST                           │
├─────────────────────────────────────────┤
│   Blockchain Mahala (Rust)              │
│   ├─ Consensus RVS                      │
│   ├─ Smart Contracts (DU)               │
│   └─ Storage Layer                      │
└─────────────────────────────────────────┘
```

---

## 📁 Structure du projet

```
mahala/
├── README.md
├── Cargo.toml                    # Workspace Rust
├── .cursorrules                  # Règles pour Cursor IDE
│
├── blockchain/                   # Coeur blockchain Rust
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   ├── block.rs             # Structure des blocs
│   │   ├── chain.rs             # Gestion de la chaîne
│   │   ├── consensus/
│   │   │   ├── mod.rs
│   │   │   ├── rvs.rs           # Random Validator Selection
│   │   │   └── vrf.rs           # Verifiable Random Function
│   │   ├── transaction.rs       # Transactions
│   │   ├── wallet.rs            # Gestion wallets
│   │   ├── du/
│   │   │   ├── mod.rs
│   │   │   └── calculator.rs    # Calcul DU selon TRM
│   │   ├── crypto/
│   │   │   ├── mod.rs
│   │   │   ├── keys.rs          # Génération clés
│   │   │   └── signatures.rs    # Signatures cryptographiques
│   │   └── storage/
│   │       ├── mod.rs
│   │       ├── checkpoint.rs    # Points de contrôle
│   │       └── merkle.rs        # Arbres de Merkle
│   └── tests/
│
├── light-client/                 # Client léger pour mobile
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   ├── sync.rs              # Synchronisation checkpoints
│   │   ├── validator.rs         # Participation consensus
│   │   ├── p2p/
│   │   │   ├── mod.rs
│   │   │   └── protocol.rs      # Protocole P2P allégé
│   │   └── storage.rs           # Stockage minimal
│   └── build.rs                 # Build config pour FFI
│
├── full-node/                    # Noeud complet
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   ├── node.rs              # Logique noeud complet
│   │   ├── mempool.rs           # Pool de transactions
│   │   ├── api/
│   │   │   ├── mod.rs
│   │   │   ├── rest.rs          # API REST (Actix-web)
│   │   │   └── websocket.rs     # WebSocket pour notifs
│   │   └── config.rs
│   └── config.toml
│
├── bridge/                       # Service bridge Mahala/June
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   ├── market_maker.rs      # AMM (Automated Market Maker)
│   │   ├── june_client.rs       # Client Duniter/June
│   │   ├── reserves.rs          # Gestion réserves
│   │   └── security.rs          # Limites et sécurité
│   └── config.toml
│
├── mobile/                       # Apps mobiles natives
│   ├── android/                 # App Android
│   │   ├── app/
│   │   │   ├── build.gradle.kts
│   │   │   ├── src/main/
│   │   │   │   ├── AndroidManifest.xml
│   │   │   │   ├── java/com/mahala/
│   │   │   │   │   ├── MainActivity.kt
│   │   │   │   │   ├── MahalaCore.kt      # Wrapper FFI Rust
│   │   │   │   │   ├── wallet/
│   │   │   │   │   │   ├── WalletActivity.kt
│   │   │   │   │   │   └── WalletViewModel.kt
│   │   │   │   │   ├── biometric/
│   │   │   │   │   │   ├── BiometricManager.kt
│   │   │   │   │   │   └── BiometricAuthenticator.kt
│   │   │   │   │   ├── exchange/
│   │   │   │   │   │   └── BridgeActivity.kt
│   │   │   │   │   ├── validator/
│   │   │   │   │   │   ├── ValidatorService.kt  # Background service
│   │   │   │   │   │   └── ValidatorNotification.kt
│   │   │   │   │   └── network/
│   │   │   │   │       └── P2PService.kt
│   │   │   │   └── res/
│   │   │   └── libs/
│   │   │       └── libmahala.so          # Lib Rust compilée
│   │   ├── build.gradle.kts
│   │   └── settings.gradle.kts
│   │
│   └── ios/                     # App iOS
│       ├── Mahala.xcodeproj
│       ├── Mahala/
│       │   ├── Info.plist
│       │   ├── MahalaApp.swift
│       │   ├── MahalaCore.swift          # Wrapper FFI Rust
│       │   ├── Views/
│       │   │   ├── ContentView.swift
│       │   │   ├── WalletView.swift
│       │   │   ├── ExchangeView.swift
│       │   │   └── ValidatorView.swift
│       │   ├── ViewModels/
│       │   │   ├── WalletViewModel.swift
│       │   │   └── ExchangeViewModel.swift
│       │   ├── Services/
│       │   │   ├── BiometricService.swift
│       │   │   ├── ValidatorService.swift
│       │   │   └── P2PService.swift
│       │   └── Frameworks/
│       │       └── libmahala.a            # Lib Rust compilée
│       └── Podfile
│
├── ffi/                          # FFI (Foreign Function Interface)
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs               # Exports FFI
│   │   ├── android.rs           # Bindings Android (JNI)
│   │   └── ios.rs               # Bindings iOS
│   ├── cbindgen.toml            # Config génération headers C
│   └── build-mobile.sh          # Script compilation cross-platform
│
├── dolibarr-module/             # Module Dolibarr PHP
│   └── mahala/
│       ├── core/
│       │   └── modules/
│       │       └── modMahala.class.php
│       ├── class/
│       │   ├── wallet.class.php
│       │   ├── exchange.class.php
│       │   └── bridge_api.class.php
│       ├── sql/
│       │   └── llx_mahala_wallet.sql
│       └── lib/
│           └── mahala.lib.php
│
├── docs/                        # Documentation
│   ├── ARCHITECTURE.md
│   ├── CONSENSUS.md
│   ├── TRM.md                   # Théorie Relative de la Monnaie
│   ├── API.md
│   └── DEPLOYMENT.md
│
├── scripts/                     # Scripts utilitaires
│   ├── setup-dev.sh
│   ├── build-mobile.sh
│   ├── deploy-node.sh
│   └── test-consensus.sh
│
└── tests/                       # Tests d'intégration
    ├── integration/
    ├── load/
    └── e2e/
```

---

## 🔧 Stack technique détaillée

### Blockchain & Backend
- **Rust** (stable latest)
  - `tokio` - Runtime async
  - `actix-web` - API REST
  - `libp2p` - Réseau P2P
  - `serde` - Sérialisation
  - `blake3` - Hashing rapide
  - `ed25519-dalek` - Signatures
  - `vrf` - Verifiable Random Function
  - `rocksdb` - Base de données embarquée
  - `prost` - Protocol Buffers

### Mobile Android
- **Kotlin** (1.9+)
- **Jetpack Compose** - UI moderne
- **AndroidX Biometric** - Authentification biométrique
- **Ktor Client** - HTTP client
- **Room** - Base de données locale
- **WorkManager** - Tâches background
- **Hilt** - Injection de dépendances

### Mobile iOS
- **Swift** (5.9+)
- **SwiftUI** - UI déclarative
- **LocalAuthentication** - Face ID / Touch ID
- **Combine** - Programmation réactive
- **CoreData** - Persistance
- **BackgroundTasks** - Tâches background

### FFI (Rust ↔ Native)
- **uniffi-rs** - Génération bindings automatique
- **JNI** (Java Native Interface) pour Android
- **cbindgen** - Headers C pour iOS

---

## 📝 Fichier .cursorrules

```yaml
# Règles Cursor pour le projet Mahala

## Conventions de code

### Rust
- Utiliser `rustfmt` avec configuration par défaut
- Suivre les conventions Rust 2021 edition
- Toujours gérer les erreurs avec `Result<T, E>`
- Préférer `async/await` pour les opérations IO
- Documentation obligatoire pour fonctions publiques
- Tests unitaires pour toute logique métier

### Kotlin
- Style officiel Kotlin
- Utiliser Coroutines pour async
- Flow pour streams réactifs
- Sealed classes pour états
- Data classes pour modèles

### Swift
- Suivre Swift API Design Guidelines
- Utiliser async/await (iOS 15+)
- Combine pour réactivité
- Structs par défaut, classes si nécessaire
- Guard pour early returns

## Architecture

### Séparation des responsabilités
- Blockchain core = logique pure sans IO
- Light client = sync + validation
- Mobile apps = UI + intégration système
- FFI = interface mince, pas de logique

### Gestion des erreurs
- Rust: Result<T, E> partout
- Mobile: Exceptions gérées avec try/catch
- FFI: Codes d'erreur + messages

### Performance
- Éviter allocations inutiles
- Utiliser pools d'objets si nécessaire
- Profiling régulier (cargo flamegraph)
- Optimiser taille binaires mobiles

### Sécurité
- Toujours valider inputs
- Pas de données sensibles en logs
- Crypto audité (dalek suite)
- Biométrie avec fallback PIN

## Tests

### Obligatoires
- Unit tests pour fonctions critiques
- Integration tests pour P2P
- Property-based testing pour consensus
- UI tests pour flows critiques mobile

### Coverage minimum
- Blockchain core: 80%
- Light client: 70%
- Mobile: 60%

## Git workflow

### Branches
- `main` - production
- `develop` - développement
- `feature/*` - nouvelles fonctionnalités
- `fix/*` - corrections bugs

### Commits
- Messages clairs et descriptifs
- Conventional commits (feat, fix, docs, etc.)
- Référencer issues si applicable

## Priorités développement

1. Blockchain core fonctionnel
2. Consensus RVS minimal
3. Light client avec sync
4. FFI + app Android basique
5. Biométrie
6. App iOS
7. Bridge June
8. Module Dolibarr

## Commandes utiles

```bash
# Build blockchain
cd blockchain && cargo build --release

# Tests
cargo test --all

# Build FFI pour Android
./ffi/build-mobile.sh android

# Build FFI pour iOS
./ffi/build-mobile.sh ios

# Run full node
cd full-node && cargo run --release

# Format code
cargo fmt --all
rustup run stable cargo clippy --all-targets --all-features
```

## Documentation

- Commenter code complexe
- README.md à jour dans chaque crate
- Architecture decisions records (ADR) dans docs/
- API documentation avec cargo doc
```

---

## 🚀 Plan de développement par phases

### Phase 1: Blockchain Core (Semaines 1-4)

#### Objectifs
- Structures de données blockchain fonctionnelles
- Consensus RVS basique
- Calcul DU conforme TRM
- Tests unitaires complets

#### Étapes dans Cursor

**1. Initialiser le workspace Rust**
```bash
# Terminal Cursor
cargo new --lib blockchain
cargo new --lib light-client
cargo new --bin full-node
cargo new --lib ffi

# Créer Cargo.toml workspace racine
```

**2. Développer blockchain/src/block.rs**
```rust
// Définir structure Block
// - header (hash précédent, timestamp, validateurs)
// - transactions
// - merkle root
// - signatures validateurs
```

**3. Développer blockchain/src/consensus/rvs.rs**
```rust
// Implémenter Random Validator Selection
// - VRF pour sélection aléatoire
// - Quorum 67%
// - Rotation toutes les 5 secondes
```

**4. Développer blockchain/src/du/calculator.rs**
```rust
// Implémenter calcul DU selon TRM
// - c = 4.88% par semestre
// - Réévaluation tous les 183 jours
// - Distribution quotidienne
```

**5. Tests**
```bash
cd blockchain && cargo test
```

#### Livrables
- ✅ Blockchain fonctionnelle en mémoire
- ✅ Consensus RVS testable
- ✅ DU calculé correctement
- ✅ 80%+ code coverage

---

### Phase 2: Light Client (Semaines 5-7)

#### Objectifs
- Client léger synchronisable
- Protocole P2P minimal
- Stockage optimisé (<50MB)
- Participation au consensus

#### Étapes

**1. light-client/src/sync.rs**
```rust
// Sync par checkpoints
// - Télécharger headers uniquement
// - Merkle proofs pour transactions personnelles
// - Validation SPV (Simplified Payment Verification)
```

**2. light-client/src/p2p/protocol.rs**
```rust
// Protocole P2P avec libp2p
// - Discovery de peers
// - Gossip léger
// - Request/Response pour sync
```

**3. light-client/src/validator.rs**
```rust
// Participation consensus
// - Écoute sélection RVS
// - Signature de bloc si sélectionné
// - Pénalités si absence
```

**4. Tests réseau**
```bash
# Lancer 10 instances light-client
./scripts/test-consensus.sh
```

#### Livrables
- ✅ Light client connecté au réseau
- ✅ Sync fonctionnel en <10 sec
- ✅ Participation consensus validée
- ✅ Consommation RAM <100MB

---

### Phase 3: FFI & App Android (Semaines 8-11)

#### Objectifs
- FFI Rust ↔ Kotlin fonctionnel
- App Android avec wallet
- Biométrie intégrée
- UI moderne Jetpack Compose

#### Étapes

**1. ffi/src/lib.rs - Exports FFI**
```rust
// Avec uniffi-rs
use uniffi;

#[uniffi::export]
pub fn create_wallet(biometric_hash: String) -> Wallet {
    // Créer wallet à partir hash biométrique
}

#[uniffi::export]
pub fn get_balance(address: String) -> f64 {
    // Obtenir balance
}

#[uniffi::export]
pub fn send_transaction(from: String, to: String, amount: f64) -> Result<String> {
    // Envoyer transaction
}

#[uniffi::export]
pub fn participate_consensus() -> bool {
    // Participer au consensus
}
```

**2. Build lib pour Android**
```bash
# ffi/build-mobile.sh
#!/bin/bash

# Android
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add x86_64-linux-android

cargo build --target aarch64-linux-android --release
cargo build --target armv7-linux-androideabi --release
cargo build --target x86_64-linux-android --release

# Copier vers Android project
cp target/aarch64-linux-android/release/libmahala.so \
   mobile/android/app/src/main/jniLibs/arm64-v8a/

cp target/armv7-linux-androideabi/release/libmahala.so \
   mobile/android/app/src/main/jniLibs/armeabi-v7a/

cp target/x86_64-linux-android/release/libmahala.so \
   mobile/android/app/src/main/jniLibs/x86_64/
```

**3. Android - MahalaCore.kt wrapper**
```kotlin
package com.mahala

// Généré automatiquement par uniffi-bindgen
object MahalaCore {
    init {
        System.loadLibrary("mahala")
    }
    
    external fun createWallet(biometricHash: String): Wallet
    external fun getBalance(address: String): Double
    external fun sendTransaction(from: String, to: String, amount: Double): String
    external fun participateConsensus(): Boolean
}
```

**4. Android - BiometricManager.kt**
```kotlin
package com.mahala.biometric

import androidx.biometric.BiometricPrompt
import androidx.biometric.BiometricManager as AndroidBiometric

class BiometricManager(private val activity: FragmentActivity) {
    
    private val biometricManager = AndroidBiometric.from(activity)
    
    fun isAvailable(): Boolean {
        return when (biometricManager.canAuthenticate(
            AndroidBiometric.Authenticators.BIOMETRIC_STRONG
        )) {
            AndroidBiometric.BIOMETRIC_SUCCESS -> true
            else -> false
        }
    }
    
    fun authenticate(onSuccess: (String) -> Unit, onError: (String) -> Unit) {
        val promptInfo = BiometricPrompt.PromptInfo.Builder()
            .setTitle("Authentification Mahala")
            .setSubtitle("Utilisez votre biométrie")
            .setAllowedAuthenticators(
                AndroidBiometric.Authenticators.BIOMETRIC_STRONG or
                AndroidBiometric.Authenticators.DEVICE_CREDENTIAL
            )
            .build()
        
        val biometricPrompt = BiometricPrompt(
            activity,
            object : BiometricPrompt.AuthenticationCallback() {
                override fun onAuthenticationSucceeded(
                    result: BiometricPrompt.AuthenticationResult
                ) {
                    // Générer hash unique de la biométrie
                    val hash = generateBiometricHash(result)
                    onSuccess(hash)
                }
                
                override fun onAuthenticationError(errorCode: Int, errString: CharSequence) {
                    onError(errString.toString())
                }
            }
        )
        
        biometricPrompt.authenticate(promptInfo)
    }
    
    private fun generateBiometricHash(result: BiometricPrompt.AuthenticationResult): String {
        // Utiliser CryptoObject pour générer hash unique
        // Sans stocker données biométriques réelles
        val crypto = result.cryptoObject
        // ... génération hash sécurisé
        return "hash_unique_irreversible"
    }
}
```

**5. Android - WalletActivity.kt**
```kotlin
package com.mahala.wallet

import androidx.compose.runtime.*
import androidx.compose.material3.*

@Composable
fun WalletScreen(viewModel: WalletViewModel) {
    val balance by viewModel.balance.collectAsState()
    val duToday by viewModel.duToday.collectAsState()
    
    Scaffold(
        topBar = { TopAppBar(title = { Text("Mon Wallet Mahala") }) }
    ) { padding ->
        Column(
            modifier = Modifier
                .padding(padding)
                .fillMaxSize()
        ) {
            // Balance
            Card(modifier = Modifier.fillMaxWidth()) {
                Column(modifier = Modifier.padding(16.dp)) {
                    Text("Solde Mahala", style = MaterialTheme.typography.titleMedium)
                    Text(
                        "💎 ${balance.format(2)} M",
                        style = MaterialTheme.typography.headlineLarge
                    )
                }
            }
            
            Spacer(modifier = Modifier.height(16.dp))
            
            // DU du jour
            Card(modifier = Modifier.fillMaxWidth()) {
                Column(modifier = Modifier.padding(16.dp)) {
                    Text("Dividende Universel", style = MaterialTheme.typography.titleMedium)
                    Text(
                        "Aujourd'hui: +${duToday.format(2)} M",
                        style = MaterialTheme.typography.bodyLarge,
                        color = MaterialTheme.colorScheme.primary
                    )
                }
            }
            
            Spacer(modifier = Modifier.height(16.dp))
            
            // Boutons actions
            Button(
                onClick = { viewModel.sendPayment() },
                modifier = Modifier.fillMaxWidth()
            ) {
                Text("Envoyer Mahala")
            }
            
            Spacer(modifier = Modifier.height(8.dp))
            
            OutlinedButton(
                onClick = { viewModel.openBridge() },
                modifier = Modifier.fillMaxWidth()
            ) {
                Text("Échanger avec June")
            }
        }
    }
}
```

**6. Android - ValidatorService.kt**
```kotlin
package com.mahala.validator

import android.app.Service
import android.content.Intent
import kotlinx.coroutines.*

class ValidatorService : Service() {
    
    private val scope = CoroutineScope(Dispatchers.Default + Job())
    
    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        // Participer au consensus en background
        scope.launch {
            while (isActive) {
                val selected = MahalaCore.checkIfSelectedValidator()
                
                if (selected) {
                    // Notifier l'utilisateur
                    showNotification("Validation requise !")
                    
                    // Signer automatiquement si app au premier plan
                    if (isAppInForeground()) {
                        MahalaCore.signBlock()
                    }
                }
                
                delay(5000) // Check toutes les 5 sec
            }
        }
        
        return START_STICKY
    }
    
    private fun showNotification(message: String) {
        // Notification critique pour validation
        val notification = NotificationCompat.Builder(this, CHANNEL_ID)
            .setContentTitle("Mahala Validator")
            .setContentText(message)
            .setPriority(NotificationCompat.PRIORITY_HIGH)
            .setCategory(NotificationCompat.CATEGORY_ALARM)
            .build()
        
        notificationManager.notify(VALIDATOR_NOTIF_ID, notification)
    }
}
```

#### Livrables
- ✅ App Android fonctionnelle
- ✅ Wallet créé avec biométrie
- ✅ Envoi/réception Mahala
- ✅ Participation consensus background
- ✅ UI Material 3

---

### Phase 4: App iOS (Semaines 12-14)

#### Objectifs
- Port iOS de l'app Android
- Face ID / Touch ID intégré
- Même fonctionnalités

#### Étapes

**1. Build lib pour iOS**
```bash
# ffi/build-mobile.sh
rustup target add aarch64-apple-ios
rustup target add x86_64-apple-ios # Simulateur

cargo build --target aarch64-apple-ios --release
cargo build --target x86_64-apple-ios --release

# Créer XCFramework
xcodebuild -create-xcframework \
  -library target/aarch64-apple-ios/release/libmahala.a \
  -library target/x86_64-apple-ios/release/libmahala.a \
  -output mobile/ios/Mahala/Frameworks/Mahala.xcframework
```

**2. iOS - MahalaCore.swift wrapper**
```swift
import Foundation

class MahalaCore {
    // FFI vers Rust
    private static func loadLibrary() {
        // Charger libmahala
    }
    
    static func createWallet(biometricHash: String) -> Wallet {
        return mahala_create_wallet(biometricHash)
    }
    
    static func getBalance(address: String) -> Double {
        return mahala_get_balance(address)
    }
    
    static func sendTransaction(from: String, to: String, amount: Double) -> String {
        return mahala_send_transaction(from, to, amount)
    }
    
    static func participateConsensus() -> Bool {
        return mahala_participate_consensus()
    }
}
```

**3. iOS - BiometricService.swift**
```swift
import LocalAuthentication

class BiometricService {
    
    private let context = LAContext()
    
    func isAvailable() -> Bool {
        var error: NSError?
        return context.canEvaluatePolicy(.deviceOwnerAuthenticationWithBiometrics, error: &error)
    }
    
    func authenticate(completion: @escaping (Result<String, Error>) -> Void) {
        var error: NSError?
        
        guard context.canEvaluatePolicy(.deviceOwnerAuthenticationWithBiometrics, error: &error) else {
            completion(.failure(error ?? BiometricError.notAvailable))
            return
        }
        
        context.evaluatePolicy(
            .deviceOwnerAuthenticationWithBiometrics,
            localizedReason: "Authentification Mahala"
        ) { success, error in
            if success {
                // Générer hash unique
                let hash = self.generateBiometricHash()
                completion(.success(hash))
            } else {
                completion(.failure(error ?? BiometricError.failed))
            }
        }
    }
    
    private func generateBiometricHash() -> String {
        // Utiliser Secure Enclave pour générer hash unique
        // Sans stocker données biométriques
        let keychain = KeychainService()
        return keychain.getBiometricHash() ?? keychain.createBiometricHash()
    }
}
```

**4. iOS - WalletView.swift**
```swift
import SwiftUI

struct WalletView: View {
    @StateObject private var viewModel = WalletViewModel()
    
    var body: some View {
        ScrollView {
            VStack(spacing: 16) {
                // Balance
                VStack(alignment: .leading, spacing: 8) {
                    Text("Solde Mahala")
                        .font(.headline)
                    
                    Text("💎 \(viewModel.balance, specifier: "%.2f") M")
                        .font(.largeTitle)
                        .fontWeight(.bold)
                }
                .frame(maxWidth: .infinity, alignment: .leading)
                .padding()
                .background(Color.blue.opacity(0.1))
                .cornerRadius(12)
                
                // DU
                VStack(alignment: .leading, spacing: 8) {
                    Text("Dividende Universel")
                        .font(.headline)
                    
                    Text("Aujourd'hui: +\(viewModel.duToday, specifier: "%.2f") M")
                        .font(.body)
                        .foregroundColor(.green)
                }
                .frame(maxWidth: .infinity, alignment: .leading)
                .padding()
                .background(Color.green.opacity(0.1))
                .cornerRadius(12)
                
                // Actions
                Button(action: { viewModel.sendPayment() }) {
                    Text("Envoyer Mahala")
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(Color.blue)
                        .foregroundColor(.white)
                        .cornerRadius(8)
                }
                
                Button(action: { viewModel.openBridge() }) {
                    Text("Échanger avec June")
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(Color.white)
                        .foregroundColor(.blue)
                        .overlay(
                            RoundedRectangle(cornerRadius: 8)
                                .stroke(Color.blue, lineWidth: 2)
                        )
                }
            }
            .padding()
        }
        .navigationTitle("Mon Wallet")
    }
}
```

**5. iOS - ValidatorService.swift**
```swift
import BackgroundTasks

class ValidatorService {
    
    static let shared = ValidatorService()
    private let bgTaskId = "com.mahala.validator"
    
    func register() {
        BGTaskScheduler.shared.register(
            forTaskWithIdentifier: bgTaskId,
            using: nil
        ) { task in
            self.handleValidation(task: task as! BGProcessingTask)
        }
    }
    
    func scheduleValidation() {
        let request = BGProcessingTaskRequest(identifier: bgTaskId)
        request.requiresNetworkConnectivity = true
        request.earliestBeginDate = Date(timeIntervalSinceNow: 5)
        
        try? BGTaskScheduler.shared.submit(request)
    }
    
    private func handleValidation(task: BGProcessingTask) {
        // Vérifier si sélectionné comme validateur
        let selected = MahalaCore.checkIfSelectedValidator()
        
        if selected {
            // Notification locale
            sendNotification("Validation requise !")
            
            // Signer si possible
            if UIApplication.shared.applicationState == .active {
                MahalaCore.signBlock()
            }
        }
        
        // Replanifier
        scheduleValidation()
        task.setTaskCompleted(success: true)
    }
    
    private func sendNotification(_ message: String) {
        let content = UNMutableNotificationContent()
        content.title = "Mahala Validator"
        content.body = message
        content.sound = .defaultCritical
        content.interruptionLevel = .critical
        
        let request = UNNotificationRequest(
            identifier: UUID().uuidString,
            content: content,
            trigger: nil
        )
        
        UNUserNotificationCenter.current().add(request)
    }
}
```

#### Livrables
- ✅ App iOS fonctionnelle
- ✅ Face ID / Touch ID
- ✅ Parité fonctionnelle avec Android
- ✅ App Store ready

---

### Phase 5: Bridge June (Semaines 15-17)

#### Objectifs
- Service bridge fonctionnel
- AMM avec réserves 10k/10k
- API REST pour apps
- Intégration apps mobiles

#### Étapes

**1. bridge/src/main.rs**
```rust
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialiser réserves
    let reserves = Reserves::new(10_000.0, 10_000.0);
    
    // Lancer serveur
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(reserves.clone()))
            .service(
                web::scope("/bridge")
                    .route("/stats", web::get().to(get_stats))
                    .route("/quote", web::post().to(get_quote))
                    .route("/exchange", web::post().to(execute_exchange))
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
```

**2. bridge/src/market_maker.rs**
```rust
pub struct MarketMaker {
    june_reserve: f64,
    mahala_reserve: f64,
    k: f64, // Constant product
    fee: f64, // 0.1%
}

impl MarketMaker {
    pub fn new(june: f64, mahala: f64) -> Self {
        Self {
            june_reserve: june,
            mahala_reserve: mahala,
            k: june * mahala,
            fee: 0.001,
        }
    }
    
    pub fn quote_mahala_to_june(&self, mahala_input: f64) -> Quote {
        let mahala_after_fee = mahala_input * (1.0 - self.fee);
        let new_mahala = self.mahala_reserve + mahala_after_fee;
        let new_june = self.k / new_mahala;
        let june_output = self.june_reserve - new_june;
        
        Quote {
            input: mahala_input,
            output: june_output,
            fee: mahala_input * self.fee,
            rate: june_output / mahala_input,
        }
    }
    
    pub fn quote_june_to_mahala(&self, june_input: f64) -> Quote {
        let june_after_fee = june_input * (1.0 - self.fee);
        let new_june = self.june_reserve + june_after_fee;
        let new_mahala = self.k / new_june;
        let mahala_output = self.mahala_reserve - new_mahala;
        
        Quote {
            input: june_input,
            output: mahala_output,
            fee: june_input * self.fee,
            rate: mahala_output / june_input,
        }
    }
    
    pub fn execute_exchange(&mut self, exchange: Exchange) -> Result<String> {
        match exchange.direction {
            Direction::MahalaToJune => {
                let quote = self.quote_mahala_to_june(exchange.amount);
                self.mahala_reserve += exchange.amount * (1.0 - self.fee);
                self.june_reserve -= quote.output;
            }
            Direction::JuneToMahala => {
                let quote = self.quote_june_to_mahala(exchange.amount);
                self.june_reserve += exchange.amount * (1.0 - self.fee);
                self.mahala_reserve -= quote.output;
            }
        }
        
        Ok("tx_hash".to_string())
    }
}
```

**3. Integration apps mobiles**
```kotlin
// Android - BridgeService.kt
class BridgeService(private val httpClient: HttpClient) {
    
    suspend fun getQuote(amount: Double, direction: String): Quote {
        return httpClient.post("$BRIDGE_URL/quote") {
            contentType(ContentType.Application.Json)
            setBody(QuoteRequest(amount, direction))
        }.body()
    }
    
    suspend fun executeExchange(amount: Double, direction: String): String {
        return httpClient.post("$BRIDGE_URL/exchange") {
            contentType(ContentType.Application.Json)
            setBody(ExchangeRequest(amount, direction))
        }.body()
    }
}
```

#### Livrables
- ✅ Bridge opérationnel
- ✅ AMM testé
- ✅ Apps intégrées
- ✅ Monitoring temps réel

---

### Phase 6: Full Node & Réseau (Semaines 18-20)

#### Objectifs
- Nœuds complets déployables
- Réseau P2P stable
- API REST complète
- Dashboard admin

#### Étapes

**1. full-node/src/main.rs**
```rust
#[tokio::main]
async fn main() -> Result<()> {
    // Initialiser node
    let config = Config::from_file("config.toml")?;
    let blockchain = Blockchain::new(&config)?;
    let p2p = P2PNetwork::new(&config).await?;
    let api = ApiServer::new(&config);
    
    // Lancer composants
    tokio::select! {
        _ = blockchain.run() => {},
        _ = p2p.run() => {},
        _ = api.run() => {},
    }
    
    Ok(())
}
```

**2. Déploiement**
```bash
# scripts/deploy-node.sh
#!/bin/bash

# Build
cd full-node
cargo build --release

# Config systemd
sudo cp target/release/mahala-node /usr/local/bin/
sudo cp mahala-node.service /etc/systemd/system/

# Start
sudo systemctl enable mahala-node
sudo systemctl start mahala-node
```

#### Livrables
- ✅ 10+ nœuds opérationnels
- ✅ Réseau stable
- ✅ Documentation déploiement

---

### Phase 7: Module Dolibarr (Semaines 21-22)

#### Objectifs
- Module Dolibarr installable
- Gestion wallets Mahala + June
- Facturation bimonétaire
- Dashboard entreprise

#### Étapes

**1. Module structure**
```php
// dolibarr-module/mahala/core/modules/modMahala.class.php
class modMahala extends DolibarrModules {
    public function __construct($db) {
        $this->numero = 500000;
        $this->name = 'Mahala';
        $this->family = "financial";
        $this->description = "Gestion Mahala/June";
        $this->version = '1.0.0';
    }
}
```

**2. Widget Dolibarr**
```php
// Afficher balance dans dashboard
class MahalaWidget {
    public function render() {
        $wallet = new Wallet($this->db);
        $wallet->fetch($user->id);
        $wallet->syncBalances();
        
        include DOL_DOCUMENT_ROOT.'/custom/mahala/tpl/widget.tpl.php';
    }
}
```

#### Livrables
- ✅ Module installable
- ✅ Docs utilisateur
- ✅ Tests avec Ma Gestion Cloud

---

## 📊 Métriques de succès

### Performance
- [ ] Temps de sync light client: <10 sec
- [ ] RAM light client: <100 MB
- [ ] Batterie/jour: <5%
- [ ] Temps de bloc: 5 sec stable
- [ ] TPS: >1000

### Qualité
- [ ] Code coverage: >70%
- [ ] Zero crash en production
- [ ] Tous audits sécurité passés
- [ ] Documentation complète

### Adoption
- [ ] 100 testeurs phase beta
- [ ] 1000 utilisateurs mois 1
- [ ] 10 commerces acceptant Mahala
- [ ] Bridge liquidité >50k June/Mahala

---

## 🔒 Checklist sécurité

### Cryptographie
- [ ] Ed25519 pour signatures
- [ ] Blake3 pour hashing
- [ ] VRF audité
- [ ] Pas de crypto custom

### Biométrie
- [ ] Données jamais transmises
- [ ] Hash one-way uniquement
- [ ] Stockage Secure Enclave/TEE
- [ ] Liveness detection

### Réseau
- [ ] TLS 1.3 pour API
- [ ] Noise protocol pour P2P
- [ ] Rate limiting
- [ ] DDoS protection

### Code
- [ ] Audit externe avant mainnet
- [ ] Bug bounty program
- [ ] Fuzzing continu
- [ ] Monitoring 24/7

---

## 📚 Ressources & Documentation

### Apprendre Rust
- The Rust Book: https://doc.rust-lang.org/book/
- Rust by Example: https://doc.rust-lang.org/rust-by-example/
- Tokio Tutorial: https://tokio.rs/tokio/tutorial

### Blockchain
- libp2p docs: https://docs.libp2p.io/
- Substrate: https://docs.substrate.io/

### Mobile
- Kotlin docs: https://kotlinlang.org/docs/home.html
- Swift docs: https://swift.org/documentation/
- uniffi-rs: https://mozilla.github.io/uniffi-rs/

### TRM
- Théorie Relative de la Monnaie: https://trm.creationmonetaire.info/

---

## 🎯 Prochaines actions Cursor

1. **Créer la structure du projet**
   ```bash
   ./scripts/setup-dev.sh
   ```

2. **Commencer par blockchain core**
   - Ouvrir `blockchain/src/lib.rs`
   - Implémenter structures de base
   - Lancer tests: `cargo test`

3. **Itérer rapidement**
   - TDD: tests d'abord
   - Commits fréquents
   - CI/CD dès le début

4. **Utiliser Cursor AI**
   - Demander génération code boilerplate
   - Refactoring assisté
   - Documentation automatique

---

## 💬 Support & Communication

- **Issues GitHub**: Pour bugs et features
- **Discord/Matrix**: Pour discussions techniques
- **Forum Mahala**: Pour communauté
- **Email**: team@mahala.org

---

**Version**: 1.0  
**Dernière mise à jour**: 2025-11-11  
**Auteur**: Équipe Mahala / Aplose
