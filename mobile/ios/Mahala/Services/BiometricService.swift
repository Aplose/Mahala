//
//  BiometricService.swift
//  Mahala
//
//  Service de gestion biométrique pour création de wallet
//

import Foundation
import LocalAuthentication

/// Service de gestion biométrique
class BiometricService {
    
    private let context = LAContext()
    
    /// Vérifier si la biométrie est disponible
    func isAvailable() -> Bool {
        var error: NSError?
        return context.canEvaluatePolicy(
            .deviceOwnerAuthenticationWithBiometrics,
            error: &error
        )
    }
    
    /// Type de biométrie disponible
    func biometricType() -> LABiometryType {
        var error: NSError?
        if context.canEvaluatePolicy(
            .deviceOwnerAuthenticationWithBiometrics,
            error: &error
        ) {
            return context.biometryType
        }
        return .none
    }
    
    /// Authentifier et générer un hash biométrique
    func authenticate() async throws -> String {
        var error: NSError?
        
        guard context.canEvaluatePolicy(
            .deviceOwnerAuthenticationWithBiometrics,
            error: &error
        ) else {
            throw BiometricError.notAvailable
        }
        
        let reason = "Créer votre portefeuille Mahala de manière sécurisée"
        
        return try await withCheckedThrowingContinuation { continuation in
            context.evaluatePolicy(
                .deviceOwnerAuthenticationWithBiometrics,
                localizedReason: reason
            ) { success, error in
                if success {
                    // Générer un hash unique et irréversible
                    let hash = self.generateBiometricHash()
                    continuation.resume(returning: hash)
                } else {
                    continuation.resume(
                        throwing: error ?? BiometricError.failed
                    )
                }
            }
        }
    }
    
    /// Générer un hash biométrique déterministe
    private func generateBiometricHash() -> String {
        // Récupérer un identifiant unique du device
        let deviceId = UIDevice.current.identifierForVendor?.uuidString ?? "default"
        
        // Récupérer ou créer un salt unique
        let keychain = KeychainService()
        var salt = keychain.getBiometricSalt()
        
        if salt == nil {
            salt = generateRandomSalt()
            keychain.saveBiometricSalt(salt!)
        }
        
        // Combiner les données pour créer un hash
        let data = "\(deviceId):\(salt!)"
        let hashData = data.data(using: .utf8)!
        
        // Hasher avec SHA-256
        var digest = [UInt8](repeating: 0, count: Int(CC_SHA256_DIGEST_LENGTH))
        hashData.withUnsafeBytes { bytes in
            _ = CC_SHA256(bytes.baseAddress, CC_LONG(hashData.count), &digest)
        }
        
        // Convertir en hexadécimal (64 caractères = 32 bytes)
        return digest.map { String(format: "%02x", $0) }.joined()
    }
    
    /// Générer un salt aléatoire
    private func generateRandomSalt() -> String {
        let characters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        return String((0..<32).map { _ in characters.randomElement()! })
    }
}

/// Erreurs biométriques
enum BiometricError: LocalizedError {
    case notAvailable
    case failed
    case userCancel
    
    var errorDescription: String? {
        switch self {
        case .notAvailable:
            return "La biométrie n'est pas disponible sur cet appareil"
        case .failed:
            return "L'authentification biométrique a échoué"
        case .userCancel:
            return "L'authentification a été annulée"
        }
    }
}

/// Service Keychain pour stockage sécurisé
class KeychainService {
    private let service = "com.mahala.biometric"
    private let saltKey = "biometric_salt"
    
    func saveBiometricSalt(_ salt: String) {
        let data = salt.data(using: .utf8)!
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: service,
            kSecAttrAccount as String: saltKey,
            kSecValueData as String: data
        ]
        
        // Supprimer l'ancienne valeur si elle existe
        SecItemDelete(query as CFDictionary)
        
        // Ajouter la nouvelle valeur
        SecItemAdd(query as CFDictionary, nil)
    }
    
    func getBiometricSalt() -> String? {
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: service,
            kSecAttrAccount as String: saltKey,
            kSecReturnData as String: true
        ]
        
        var result: AnyObject?
        let status = SecItemCopyMatching(query as CFDictionary, &result)
        
        if status == errSecSuccess,
           let data = result as? Data,
           let salt = String(data: data, encoding: .utf8) {
            return salt
        }
        
        return nil
    }
}

// Import pour SHA-256
import CommonCrypto

