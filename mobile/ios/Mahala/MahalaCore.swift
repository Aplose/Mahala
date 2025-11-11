//
//  MahalaCore.swift
//  Mahala
//
//  Wrapper Swift pour la bibliothèque Rust Mahala
//

import Foundation

@_silgen_name("init_light_client")
func init_light_client(_ url: UnsafePointer<CChar>) -> Int32

@_silgen_name("create_wallet_from_biometric")
func create_wallet_from_biometric(
    _ hash: UnsafePointer<CChar>,
    _ output: UnsafeMutablePointer<CChar>,
    _ outputLen: Int32
) -> Int32

@_silgen_name("get_balance")
func get_balance() -> Double

@_silgen_name("get_daily_du")
func get_daily_du() -> Double

@_silgen_name("check_if_selected_validator")
func check_if_selected_validator() -> Int32

@_silgen_name("sync")
func sync() -> Int32

/// Wrapper Swift pour Mahala Core
class MahalaCore {
    
    /// Initialiser le light client
    static func initLightClient(fullNodeUrl: String) -> Bool {
        let urlCString = fullNodeUrl.cString(using: .utf8)!
        let result = init_light_client(urlCString)
        return result == 0
    }
    
    /// Créer un wallet depuis un hash biométrique
    static func createWalletFromBiometric(biometricHashHex: String) -> Result<String, Error> {
        let hashCString = biometricHashHex.cString(using: .utf8)!
        let output = UnsafeMutablePointer<CChar>.allocate(capacity: 128)
        defer { output.deallocate() }
        
        let result = create_wallet_from_biometric(hashCString, output, 128)
        
        if result == 0 {
            let address = String(cString: output)
            return .success(address)
        } else {
            return .failure(NSError(
                domain: "MahalaCore",
                code: Int(result),
                userInfo: [NSLocalizedDescriptionKey: "Failed to create wallet: error code \(result)"]
            ))
        }
    }
    
    /// Obtenir la balance
    static func getBalance() -> Double {
        return get_balance()
    }
    
    /// Obtenir le Dividende Universel du jour
    static func getDailyDu() -> Double {
        return get_daily_du()
    }
    
    /// Vérifier si sélectionné comme validateur
    static func checkIfSelectedValidator() -> Bool {
        return check_if_selected_validator() == 1
    }
    
    /// Synchroniser avec le réseau
    static func sync() -> Bool {
        return sync() == 0
    }
    
    // Versions async avec Task
    
    /// Créer un wallet (async)
    static func createWalletFromBiometricAsync(biometricHashHex: String) async throws -> String {
        return try await withCheckedThrowingContinuation { continuation in
            Task.detached {
                let result = createWalletFromBiometric(biometricHashHex: biometricHashHex)
                continuation.resume(with: result)
            }
        }
    }
    
    /// Obtenir la balance (async)
    static func getBalanceAsync() async -> Double {
        return await Task.detached {
            getBalance()
        }.value
    }
    
    /// Obtenir le DU (async)
    static func getDailyDuAsync() async -> Double {
        return await Task.detached {
            getDailyDu()
        }.value
    }
    
    /// Vérifier si sélectionné (async)
    static func checkIfSelectedValidatorAsync() async -> Bool {
        return await Task.detached {
            checkIfSelectedValidator()
        }.value
    }
    
    /// Synchroniser (async)
    static func syncAsync() async -> Bool {
        return await Task.detached {
            sync()
        }.value
    }
}

