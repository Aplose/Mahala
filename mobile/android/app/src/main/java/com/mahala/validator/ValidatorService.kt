package com.mahala.validator

import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.app.Service
import android.content.Context
import android.content.Intent
import android.os.Build
import android.os.IBinder
import androidx.core.app.NotificationCompat
import androidx.work.*
import com.mahala.MainActivity
import com.mahala.MahalaCore
import com.mahala.R
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch
import java.util.concurrent.TimeUnit

/**
 * Service en arrière-plan pour participer au consensus et recevoir le DU
 * 
 * Ce service doit tourner en continu pour que l'utilisateur bénéficie
 * du Dividende Universel et participe à la validation des blocs.
 */
class ValidatorService : Service() {
    
    private val serviceScope = CoroutineScope(SupervisorJob() + Dispatchers.Default)
    private val CHANNEL_ID = "mahala_validator_channel"
    private val NOTIFICATION_ID = 1

    override fun onCreate() {
        super.onCreate()
        createNotificationChannel()
        startForeground(NOTIFICATION_ID, createNotification("Service actif"))
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        // Démarrer la participation au consensus
        serviceScope.launch {
            participateInConsensus()
        }

        // Démarrer la synchronisation périodique
        schedulePeriodicSync()

        return START_STICKY // Redémarrer si tué par le système
    }

    override fun onBind(intent: Intent?): IBinder? = null

    /**
     * Participer au consensus RVS
     */
    private suspend fun participateInConsensus() {
        while (true) {
            try {
                // Vérifier si ce wallet est sélectionné comme validateur
                MahalaCore.checkIfSelectedValidatorAsync()
                    .onSuccess { selected ->
                        if (selected) {
                            // Notifier l'utilisateur
                            showValidationNotification()
                            
                            // TODO: Signer le bloc automatiquement si possible
                            // (nécessite la clé privée, à gérer avec sécurité)
                        }
                    }

                // Vérifier toutes les 5 secondes (intervalle des blocs)
                delay(5000)
            } catch (e: Exception) {
                // Log l'erreur et continuer
                delay(5000)
            }
        }
    }

    /**
     * Planifier la synchronisation périodique
     */
    private fun schedulePeriodicSync() {
        val constraints = Constraints.Builder()
            .setRequiredNetworkType(NetworkType.CONNECTED)
            .setRequiresBatteryNotLow(true)
            .build()

        val syncWork = PeriodicWorkRequestBuilder<SyncWorker>(
            15, TimeUnit.MINUTES, // Toutes les 15 minutes
            5, TimeUnit.MINUTES   // Flexibilité de 5 minutes
        )
            .setConstraints(constraints)
            .build()

        WorkManager.getInstance(this)
            .enqueueUniquePeriodicWork(
                "mahala_sync",
                ExistingPeriodicWorkPolicy.KEEP,
                syncWork
            )
    }

    /**
     * Créer le canal de notification
     */
    private fun createNotificationChannel() {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            val channel = NotificationChannel(
                CHANNEL_ID,
                "Mahala Validator",
                NotificationManager.IMPORTANCE_LOW
            ).apply {
                description = "Notifications pour la participation au consensus Mahala"
            }

            val notificationManager = getSystemService(NotificationManager::class.java)
            notificationManager.createNotificationChannel(channel)
        }
    }

    /**
     * Créer une notification
     */
    private fun createNotification(text: String): android.app.Notification {
        val intent = Intent(this, MainActivity::class.java)
        val pendingIntent = PendingIntent.getActivity(
            this, 0, intent,
            PendingIntent.FLAG_IMMUTABLE
        )

        return NotificationCompat.Builder(this, CHANNEL_ID)
            .setContentTitle("Mahala Validator")
            .setContentText(text)
            .setSmallIcon(android.R.drawable.ic_dialog_info)
            .setContentIntent(pendingIntent)
            .setOngoing(true)
            .build()
    }

    /**
     * Afficher une notification de validation requise
     */
    private fun showValidationNotification() {
        val notification = createNotification("Validation de bloc requise !")
        val notificationManager = getSystemService(NotificationManager::class.java)
        notificationManager.notify(NOTIFICATION_ID + 1, notification)
    }

    override fun onDestroy() {
        super.onDestroy()
        serviceScope.cancel()
    }
}

/**
 * Worker pour la synchronisation périodique
 */
class SyncWorker(context: Context, params: WorkerParameters) : CoroutineWorker(context, params) {
    override suspend fun doWork(): Result {
        return try {
            MahalaCore.syncAsync()
            Result.success()
        } catch (e: Exception) {
            Result.retry()
        }
    }
}

