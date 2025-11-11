//
//  ValidatorService.swift
//  Mahala
//
//  Service en arrière-plan pour participer au consensus et recevoir le DU
//

import Foundation
import BackgroundTasks
import UserNotifications

/// Service de validation en arrière-plan
class ValidatorService {
    
    static let shared = ValidatorService()
    
    private let taskIdentifier = "com.mahala.validator"
    private var backgroundTask: UIBackgroundTaskIdentifier = .invalid
    
    private init() {}
    
    /// Enregistrer les tâches en arrière-plan
    func registerBackgroundTasks() {
        BGTaskScheduler.shared.register(
            forTaskWithIdentifier: taskIdentifier,
            using: nil
        ) { task in
            self.handleValidationTask(task: task as! BGProcessingTask)
        }
    }
    
    /// Planifier la prochaine validation
    func scheduleValidation() {
        let request = BGProcessingTaskRequest(identifier: taskIdentifier)
        request.requiresNetworkConnectivity = true
        request.requiresExternalPower = false
        request.earliestBeginDate = Date(timeIntervalSinceNow: 5) // Dans 5 secondes
        
        do {
            try BGTaskScheduler.shared.submit(request)
        } catch {
            print("Erreur lors de la planification: \(error)")
        }
    }
    
    /// Gérer la tâche de validation
    private func handleValidationTask(task: BGProcessingTask) {
        // Planifier la prochaine exécution
        scheduleValidation()
        
        // Vérifier si sélectionné comme validateur
        Task {
            let selected = await MahalaCore.checkIfSelectedValidatorAsync()
            
            if selected {
                // Notifier l'utilisateur
                sendNotification(
                    title: "Validation requise",
                    body: "Vous avez été sélectionné pour valider un bloc Mahala"
                )
                
                // TODO: Signer le bloc automatiquement si possible
            }
            
            // Synchroniser
            _ = await MahalaCore.syncAsync()
            
            task.setTaskCompleted(success: true)
        }
        
        // Gérer l'expiration de la tâche
        task.expirationHandler = {
            task.setTaskCompleted(success: false)
        }
    }
    
    /// Envoyer une notification
    private func sendNotification(title: String, body: String) {
        let content = UNMutableNotificationContent()
        content.title = title
        content.body = body
        content.sound = .default
        
        let request = UNNotificationRequest(
            identifier: UUID().uuidString,
            content: content,
            trigger: nil
        )
        
        UNUserNotificationCenter.current().add(request) { error in
            if let error = error {
                print("Erreur notification: \(error)")
            }
        }
    }
    
    /// Démarrer le service en arrière-plan
    func startBackgroundTask() {
        backgroundTask = UIApplication.shared.beginBackgroundTask { [weak self] in
            self?.endBackgroundTask()
        }
        
        // Planifier la première validation
        scheduleValidation()
    }
    
    /// Arrêter le service en arrière-plan
    func endBackgroundTask() {
        if backgroundTask != .invalid {
            UIApplication.shared.endBackgroundTask(backgroundTask)
            backgroundTask = .invalid
        }
    }
}

