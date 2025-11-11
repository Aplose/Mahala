//
//  WalletViewModel.swift
//  Mahala
//
//  ViewModel pour le wallet Mahala
//

import Foundation
import Combine

@MainActor
class WalletViewModel: ObservableObject {
    
    @Published var isLoading = false
    @Published var isSyncing = false
    @Published var walletCreated = false
    @Published var walletAddress = ""
    @Published var balance: Double = 0.0
    @Published var dailyDu: Double = 0.0
    @Published var error: String?
    
    private var cancellables = Set<AnyCancellable>()
    
    init() {
        // Initialiser le light client
        MahalaCore.initLightClient(fullNodeUrl: "http://node.mahala.org:8080")
        
        // Charger le wallet au démarrage
        loadWallet()
        
        // Démarrer la synchronisation périodique
        startPeriodicSync()
    }
    
    /// Charger le wallet depuis le stockage local
    private func loadWallet() {
        // TODO: Charger depuis UserDefaults ou Keychain
        // Pour l'instant, on vérifie si un wallet existe
        walletCreated = UserDefaults.standard.bool(forKey: "wallet_created")
        if walletCreated {
            walletAddress = UserDefaults.standard.string(forKey: "wallet_address") ?? ""
            refreshBalance()
            refreshDailyDu()
        }
    }
    
    /// Créer un nouveau wallet avec biométrie
    func createWallet(biometricHash: String) async {
        isLoading = true
        error = nil
        
        do {
            let address = try await MahalaCore.createWalletFromBiometricAsync(
                biometricHashHex: biometricHash
            )
            
            walletAddress = address
            walletCreated = true
            
            // Sauvegarder
            UserDefaults.standard.set(true, forKey: "wallet_created")
            UserDefaults.standard.set(address, forKey: "wallet_address")
            
            await refreshBalance()
            await refreshDailyDu()
            
            isLoading = false
        } catch {
            self.error = error.localizedDescription
            isLoading = false
        }
    }
    
    /// Rafraîchir la balance
    func refreshBalance() async {
        let newBalance = await MahalaCore.getBalanceAsync()
        balance = newBalance
    }
    
    /// Obtenir le Dividende Universel du jour
    func refreshDailyDu() async {
        let du = await MahalaCore.getDailyDuAsync()
        dailyDu = du
    }
    
    /// Synchroniser avec le réseau
    func sync() async {
        isSyncing = true
        error = nil
        
        let success = await MahalaCore.syncAsync()
        
        if success {
            await refreshBalance()
            await refreshDailyDu()
        } else {
            error = "Erreur lors de la synchronisation"
        }
        
        isSyncing = false
    }
    
    /// Démarrer la synchronisation périodique
    private func startPeriodicSync() {
        Timer.publish(every: 10, on: .main, in: .common)
            .autoconnect()
            .sink { [weak self] _ in
                guard let self = self, self.walletCreated else { return }
                Task {
                    await self.sync()
                }
            }
            .store(in: &cancellables)
    }
}

