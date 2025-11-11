//
//  WalletView.swift
//  Mahala
//
//  Vue principale du wallet
//

import SwiftUI

struct WalletView: View {
    @StateObject private var viewModel = WalletViewModel()
    @StateObject private var biometricService = BiometricService()
    
    var body: some View {
        NavigationView {
            ScrollView {
                VStack(spacing: 20) {
                    if !viewModel.walletCreated {
                        CreateWalletView(
                            viewModel: viewModel,
                            biometricService: biometricService
                        )
                    } else {
                        WalletContentView(viewModel: viewModel)
                    }
                    
                    if let error = viewModel.error {
                        ErrorView(error: error)
                    }
                }
                .padding()
            }
            .navigationTitle("Mahala Wallet")
        }
    }
}

struct CreateWalletView: View {
    @ObservedObject var viewModel: WalletViewModel
    @ObservedObject var biometricService: BiometricService
    @State private var isAuthenticating = false
    
    var body: some View {
        VStack(spacing: 20) {
            Text("Créer votre portefeuille membre")
                .font(.title2)
                .fontWeight(.bold)
            
            Text("Votre wallet sera créé de manière sécurisée à partir de votre biométrie. Les données biométriques ne sont jamais stockées.")
                .font(.body)
                .foregroundColor(.secondary)
                .multilineTextAlignment(.center)
                .padding(.horizontal)
            
            Button(action: {
                isAuthenticating = true
                Task {
                    do {
                        let hash = try await biometricService.authenticate()
                        await viewModel.createWallet(biometricHash: hash)
                        isAuthenticating = false
                    } catch {
                        viewModel.error = error.localizedDescription
                        isAuthenticating = false
                    }
                }
            }) {
                HStack {
                    if isAuthenticating {
                        ProgressView()
                            .progressViewStyle(CircularProgressViewStyle(tint: .white))
                    } else {
                        Image(systemName: "faceid")
                        Text("Créer avec biométrie")
                    }
                }
                .frame(maxWidth: .infinity)
                .padding()
                .background(Color.blue)
                .foregroundColor(.white)
                .cornerRadius(10)
            }
            .disabled(isAuthenticating || !biometricService.isAvailable())
            .padding(.horizontal)
        }
    }
}

struct WalletContentView: View {
    @ObservedObject var viewModel: WalletViewModel
    
    var body: some View {
        VStack(spacing: 20) {
            // Balance
            CardView {
                VStack(spacing: 10) {
                    Text("Solde Mahala")
                        .font(.headline)
                    Text("💎 \(String(format: "%.2f", viewModel.balance)) M")
                        .font(.system(size: 32, weight: .bold))
                }
                .frame(maxWidth: .infinity)
                .padding()
            }
            
            // Dividende Universel
            CardView {
                VStack(alignment: .leading, spacing: 10) {
                    Text("Dividende Universel")
                        .font(.headline)
                    Text("Aujourd'hui: +\(String(format: "%.4f", viewModel.dailyDu)) M")
                        .font(.body)
                        .foregroundColor(.green)
                    Text("L'app doit tourner en arrière-plan pour recevoir le DU")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }
                .frame(maxWidth: .infinity, alignment: .leading)
                .padding()
            }
            
            // Adresse du wallet
            CardView {
                VStack(alignment: .leading, spacing: 5) {
                    Text("Adresse du wallet")
                        .font(.subheadline)
                        .fontWeight(.semibold)
                    Text(String(viewModel.walletAddress.prefix(20)) + "...")
                        .font(.system(.caption, design: .monospaced))
                        .foregroundColor(.secondary)
                }
                .frame(maxWidth: .infinity, alignment: .leading)
                .padding()
            }
            
            // Boutons d'action
            HStack(spacing: 10) {
                Button(action: {
                    Task {
                        await viewModel.refreshBalance()
                        await viewModel.refreshDailyDu()
                    }
                }) {
                    Text("Actualiser")
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(Color.blue)
                        .foregroundColor(.white)
                        .cornerRadius(10)
                }
                
                Button(action: {
                    Task {
                        await viewModel.sync()
                    }
                }) {
                    HStack {
                        if viewModel.isSyncing {
                            ProgressView()
                                .progressViewStyle(CircularProgressViewStyle(tint: .white))
                        } else {
                            Text("Synchroniser")
                        }
                    }
                    .frame(maxWidth: .infinity)
                    .padding()
                    .background(Color.green)
                    .foregroundColor(.white)
                    .cornerRadius(10)
                }
                .disabled(viewModel.isSyncing)
            }
        }
    }
}

struct CardView<Content: View>: View {
    let content: Content
    
    init(@ViewBuilder content: () -> Content) {
        self.content = content()
    }
    
    var body: some View {
        content
            .background(Color(.systemGray6))
            .cornerRadius(12)
    }
}

struct ErrorView: View {
    let error: String
    
    var body: some View {
        Text(error)
            .padding()
            .background(Color.red.opacity(0.1))
            .foregroundColor(.red)
            .cornerRadius(10)
    }
}

struct WalletView_Previews: PreviewProvider {
    static var previews: some View {
        WalletView()
    }
}

