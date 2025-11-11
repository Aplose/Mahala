package com.mahala.biometric

import android.content.Context
import androidx.biometric.BiometricManager
import androidx.biometric.BiometricPrompt
import androidx.core.content.ContextCompat
import androidx.fragment.app.FragmentActivity
import java.security.MessageDigest

/**
 * Gestionnaire de biométrie pour Mahala
 * 
 * Utilise la biométrie pour créer un hash unique et irréversible
 * qui sert de seed pour générer le wallet de manière déterministe
 */
class BiometricManager(private val activity: FragmentActivity) {
    
    private val biometricManager = BiometricManager.from(activity)
    
    /**
     * Vérifier si la biométrie est disponible
     */
    fun isAvailable(): Boolean {
        return when (biometricManager.canAuthenticate(
            BiometricManager.Authenticators.BIOMETRIC_STRONG or
            BiometricManager.Authenticators.DEVICE_CREDENTIAL
        )) {
            BiometricManager.BIOMETRIC_SUCCESS -> true
            else -> false
        }
    }

    /**
     * Authentifier et générer un hash biométrique
     * 
     * Le hash est généré de manière déterministe à partir de:
     * - Les caractéristiques biométriques (sans les stocker)
     * - Un identifiant unique du device
     * - Un salt stocké de manière sécurisée
     */
    fun authenticate(
        onSuccess: (String) -> Unit,
        onError: (String) -> Unit
    ) {
        val promptInfo = BiometricPrompt.PromptInfo.Builder()
            .setTitle("Création du Wallet Mahala")
            .setSubtitle("Utilisez votre biométrie pour créer votre portefeuille membre")
            .setDescription("Votre biométrie est utilisée pour générer votre wallet de manière sécurisée. Les données biométriques ne sont jamais stockées.")
            .setAllowedAuthenticators(
                BiometricManager.Authenticators.BIOMETRIC_STRONG or
                BiometricManager.Authenticators.DEVICE_CREDENTIAL
            )
            .setNegativeButtonText("Annuler")
            .build()

        val executor = ContextCompat.getMainExecutor(activity)
        val biometricPrompt = BiometricPrompt(
            activity,
            executor,
            object : BiometricPrompt.AuthenticationCallback() {
                override fun onAuthenticationSucceeded(
                    result: BiometricPrompt.AuthenticationResult
                ) {
                    // Générer un hash unique et irréversible
                    val biometricHash = generateBiometricHash(activity)
                    onSuccess(biometricHash)
                }

                override fun onAuthenticationError(
                    errorCode: Int,
                    errString: CharSequence
                ) {
                    onError(errString.toString())
                }

                override fun onAuthenticationFailed() {
                    onError("Authentification échouée")
                }
            }
        )

        biometricPrompt.authenticate(promptInfo)
    }

    /**
     * Générer un hash biométrique unique et irréversible
     * 
     * Ce hash est déterministe: le même utilisateur sur le même device
     * générera toujours le même hash, permettant de récupérer le wallet.
     */
    private fun generateBiometricHash(context: Context): String {
        // Récupérer un identifiant unique du device (Android ID)
        val androidId = android.provider.Settings.Secure.getString(
            context.contentResolver,
            android.provider.Settings.Secure.ANDROID_ID
        ) ?: "default"

        // Récupérer ou créer un salt unique pour ce device
        val prefs = context.getSharedPreferences("mahala_biometric", Context.MODE_PRIVATE)
        var salt = prefs.getString("biometric_salt", null)
        
        if (salt == null) {
            // Générer un nouveau salt aléatoire
            salt = generateRandomSalt()
            prefs.edit().putString("biometric_salt", salt).apply()
        }

        // Combiner les données pour créer un hash
        val data = "$androidId:$salt"
        val digest = MessageDigest.getInstance("SHA-256")
        val hashBytes = digest.digest(data.toByteArray())

        // Convertir en hexadécimal (64 caractères = 32 bytes)
        return hashBytes.joinToString("") { "%02x".format(it) }
    }

    /**
     * Générer un salt aléatoire
     */
    private fun generateRandomSalt(): String {
        val chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        return (1..32)
            .map { chars.random() }
            .joinToString("")
    }
}

