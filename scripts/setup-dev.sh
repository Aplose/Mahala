#!/bin/bash
# Script de configuration de l'environnement de développement Mahala

set -e

echo "🚀 Configuration de l'environnement de développement Mahala"

# Vérifier Rust
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust n'est pas installé. Installation..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
else
    echo "✅ Rust est installé: $(rustc --version)"
fi

# Installer les targets nécessaires
echo "📦 Installation des targets Rust..."

# Android
echo "  - Targets Android..."
rustup target add aarch64-linux-android 2>/dev/null || true
rustup target add armv7-linux-androideabi 2>/dev/null || true
rustup target add x86_64-linux-android 2>/dev/null || true

# iOS (si sur macOS)
if [[ "$OSTYPE" == "darwin"* ]]; then
    echo "  - Targets iOS..."
    rustup target add aarch64-apple-ios 2>/dev/null || true
    rustup target add aarch64-apple-ios-sim 2>/dev/null || true
    rustup target add x86_64-apple-ios 2>/dev/null || true
fi

# Vérifier les dépendances système
echo "🔍 Vérification des dépendances système..."

# OpenSSL
if ! pkg-config --exists openssl 2>/dev/null; then
    echo "⚠️  OpenSSL n'est pas installé. Installation recommandée:"
    echo "   Ubuntu/Debian: sudo apt-get install libssl-dev pkg-config"
    echo "   Fedora: sudo dnf install openssl-devel pkg-config"
    echo "   macOS: brew install openssl pkg-config"
fi

# Compiler les dépendances
echo "🔨 Compilation des dépendances..."
cargo build --workspace

echo "✅ Configuration terminée!"
echo ""
echo "Prochaines étapes:"
echo "  1. Lancer un nœud: cd full-node && cargo run --release"
echo "  2. Lancer le bridge: cd bridge && cargo run --release"
echo "  3. Compiler pour mobile: voir mobile/android/README.md"

