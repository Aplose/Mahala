package com.mahala

import android.app.Application
import android.content.Intent
import com.mahala.validator.ValidatorService

/**
 * Application principale Mahala
 * 
 * Démarre automatiquement le service de validation en arrière-plan
 */
class MahalaApplication : Application() {
    
    override fun onCreate() {
        super.onCreate()
        
        // Démarrer le service de validation en arrière-plan
        // Ce service doit tourner en continu pour bénéficier du DU
        val serviceIntent = Intent(this, ValidatorService::class.java)
        if (android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.O) {
            startForegroundService(serviceIntent)
        } else {
            startService(serviceIntent)
        }
    }
}

