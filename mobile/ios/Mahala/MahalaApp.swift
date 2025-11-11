//
//  MahalaApp.swift
//  Mahala
//
//  Point d'entrée de l'application
//

import SwiftUI

@main
struct MahalaApp: App {
    
    @UIApplicationDelegateAdaptor(AppDelegate.self) var appDelegate
    
    var body: some Scene {
        WindowGroup {
            WalletView()
        }
    }
}

/// AppDelegate pour gérer les tâches en arrière-plan
class AppDelegate: NSObject, UIApplicationDelegate {
    
    func application(
        _ application: UIApplication,
        didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey : Any]? = nil
    ) -> Bool {
        // Enregistrer les tâches en arrière-plan
        ValidatorService.shared.registerBackgroundTasks()
        
        return true
    }
}

