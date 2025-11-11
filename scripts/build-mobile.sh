#!/bin/bash
# Script de compilation pour mobile (Android/iOS)

set -e

PLATFORM=$1

if [ -z "$PLATFORM" ]; then
    echo "Usage: $0 [android|ios]"
    exit 1
fi

cd "$(dirname "$0")/.."

case $PLATFORM in
    android)
        echo "📱 Compilation pour Android..."
        
        # Installer les targets si nécessaire
        rustup target add aarch64-linux-android
        rustup target add armv7-linux-androideabi
        rustup target add x86_64-linux-android
        
        # Compiler
        cd ffi
        cargo build --target aarch64-linux-android --release
        cargo build --target armv7-linux-androideabi --release
        cargo build --target x86_64-linux-android --release
        
        # Créer les répertoires
        mkdir -p ../mobile/android/app/src/main/jniLibs/{arm64-v8a,armeabi-v7a,x86_64}
        
        # Copier les bibliothèques
        cp target/aarch64-linux-android/release/libmahala.so \
           ../mobile/android/app/src/main/jniLibs/arm64-v8a/
        cp target/armv7-linux-androideabi/release/libmahala.so \
           ../mobile/android/app/src/main/jniLibs/armeabi-v7a/
        cp target/x86_64-linux-android/release/libmahala.so \
           ../mobile/android/app/src/main/jniLibs/x86_64/
        
        echo "✅ Bibliothèques Android compilées!"
        ;;
        
    ios)
        if [[ "$OSTYPE" != "darwin"* ]]; then
            echo "❌ iOS nécessite macOS et Xcode"
            exit 1
        fi
        
        echo "📱 Compilation pour iOS..."
        
        # Installer les targets
        rustup target add aarch64-apple-ios
        rustup target add aarch64-apple-ios-sim
        
        # Compiler
        cd ffi
        cargo build --target aarch64-apple-ios --release
        cargo build --target aarch64-apple-ios-sim --release
        
        # Créer le XCFramework
        xcodebuild -create-xcframework \
          -library target/aarch64-apple-ios/release/libmahala.a \
          -library target/aarch64-apple-ios-sim/release/libmahala.a \
          -output ../mobile/ios/Mahala/Frameworks/Mahala.xcframework
        
        echo "✅ Framework iOS créé!"
        ;;
        
    *)
        echo "❌ Plateforme inconnue: $PLATFORM"
        echo "Usage: $0 [android|ios]"
        exit 1
        ;;
esac

