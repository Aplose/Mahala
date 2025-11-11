package com.mahala

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext

/**
 * Wrapper JNI pour la bibliothèque Rust Mahala
 */
object MahalaCore {
    init {
        System.loadLibrary("mahala")
    }

    // Fonctions natives JNI
    @JvmStatic
    external fun initLightClient(fullNodeUrl: String): Int
    
    @JvmStatic
    external fun createWalletFromBiometric(biometricHashHex: String, output: ByteArray, outputLen: Int): Int
    
    @JvmStatic
    external fun getBalance(): Double
    
    @JvmStatic
    external fun getDailyDu(): Double
    
    @JvmStatic
    external fun checkIfSelectedValidator(): Int
    
    @JvmStatic
    external fun sync(): Int

    /**
     * Créer un wallet depuis un hash biométrique (async)
     */
    suspend fun createWalletFromBiometricAsync(biometricHashHex: String): Result<String> {
        return withContext(Dispatchers.IO) {
            try {
                val output = ByteArray(128) // Taille suffisante pour une adresse hex
                val result = createWalletFromBiometric(biometricHashHex, output, output.size)
                
                if (result == 0) {
                    // Trouver la fin de la string (null terminator)
                    val nullIndex = output.indexOf(0)
                    val addressBytes = if (nullIndex > 0) {
                        output.sliceArray(0 until nullIndex)
                    } else {
                        output
                    }
                    val address = String(addressBytes, Charsets.UTF_8)
                    Result.success(address)
                } else {
                    Result.failure(Exception("Failed to create wallet: error code $result"))
                }
            } catch (e: Exception) {
                Result.failure(e)
            }
        }
    }

    /**
     * Obtenir la balance (async)
     */
    suspend fun getBalanceAsync(): Result<Double> {
        return withContext(Dispatchers.IO) {
            try {
                val balance = getBalance()
                Result.success(balance)
            } catch (e: Exception) {
                Result.failure(e)
            }
        }
    }

    /**
     * Obtenir le DU du jour (async)
     */
    suspend fun getDailyDuAsync(): Result<Double> {
        return withContext(Dispatchers.IO) {
            try {
                val du = getDailyDu()
                Result.success(du)
            } catch (e: Exception) {
                Result.failure(e)
            }
        }
    }

    /**
     * Vérifier si sélectionné comme validateur (async)
     */
    suspend fun checkIfSelectedValidatorAsync(): Result<Boolean> {
        return withContext(Dispatchers.IO) {
            try {
                val selected = checkIfSelectedValidator() == 1
                Result.success(selected)
            } catch (e: Exception) {
                Result.failure(e)
            }
        }
    }

    /**
     * Synchroniser avec le réseau (async)
     */
    suspend fun syncAsync(): Result<Unit> {
        return withContext(Dispatchers.IO) {
            try {
                val result = sync()
                if (result == 0) {
                    Result.success(Unit)
                } else {
                    Result.failure(Exception("Sync failed with error code $result"))
                }
            } catch (e: Exception) {
                Result.failure(e)
            }
        }
    }
}
