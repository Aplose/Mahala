package com.mahala.wallet

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import com.mahala.MahalaCore

/**
 * ViewModel pour le wallet Mahala
 */
class WalletViewModel : ViewModel() {
    
    private val _uiState = MutableStateFlow(WalletUiState())
    val uiState: StateFlow<WalletUiState> = _uiState.asStateFlow()

    init {
        // Charger le wallet au démarrage
        loadWallet()
        
        // Démarrer la synchronisation périodique
        startPeriodicSync()
    }

    /**
     * Charger le wallet depuis le stockage local
     */
    private fun loadWallet() {
        viewModelScope.launch {
            // TODO: Charger depuis Room/SecureStorage
            // Pour l'instant, on vérifie si un wallet existe
            _uiState.value = _uiState.value.copy(
                isLoading = false,
                walletCreated = false // TODO: Vérifier depuis le stockage
            )
        }
    }

    /**
     * Créer un nouveau wallet avec biométrie
     */
    fun createWallet(biometricHash: String) {
        viewModelScope.launch {
            _uiState.value = _uiState.value.copy(isLoading = true)
            
            MahalaCore.createWalletFromBiometricAsync(biometricHash)
                .onSuccess { address ->
                    _uiState.value = _uiState.value.copy(
                        isLoading = false,
                        walletCreated = true,
                        walletAddress = address,
                        error = null
                    )
                    // Charger la balance
                    refreshBalance()
                }
                .onFailure { error ->
                    _uiState.value = _uiState.value.copy(
                        isLoading = false,
                        error = error.message ?: "Erreur lors de la création du wallet"
                    )
                }
        }
    }

    /**
     * Rafraîchir la balance
     */
    fun refreshBalance() {
        viewModelScope.launch {
            MahalaCore.getBalanceAsync()
                .onSuccess { balance ->
                    _uiState.value = _uiState.value.copy(
                        balance = balance,
                        error = null
                    )
                }
                .onFailure { error ->
                    _uiState.value = _uiState.value.copy(
                        error = error.message ?: "Erreur lors du chargement de la balance"
                    )
                }
        }
    }

    /**
     * Obtenir le Dividende Universel du jour
     */
    fun refreshDailyDu() {
        viewModelScope.launch {
            MahalaCore.getDailyDuAsync()
                .onSuccess { du ->
                    _uiState.value = _uiState.value.copy(
                        dailyDu = du,
                        error = null
                    )
                }
                .onFailure { error ->
                    _uiState.value = _uiState.value.copy(
                        error = error.message ?: "Erreur lors du chargement du DU"
                    )
                }
        }
    }

    /**
     * Synchroniser avec le réseau
     */
    fun sync() {
        viewModelScope.launch {
            _uiState.value = _uiState.value.copy(isSyncing = true)
            
            MahalaCore.syncAsync()
                .onSuccess {
                    _uiState.value = _uiState.value.copy(
                        isSyncing = false,
                        error = null
                    )
                    refreshBalance()
                    refreshDailyDu()
                }
                .onFailure { error ->
                    _uiState.value = _uiState.value.copy(
                        isSyncing = false,
                        error = error.message ?: "Erreur lors de la synchronisation"
                    )
                }
        }
    }

    /**
     * Démarrer la synchronisation périodique
     */
    private fun startPeriodicSync() {
        viewModelScope.launch {
            // Synchroniser toutes les 10 secondes
            while (true) {
            kotlinx.coroutines.delay(10000)
                if (_uiState.value.walletCreated) {
                    sync()
                }
            }
        }
    }
}

/**
 * État de l'UI du wallet
 */
data class WalletUiState(
    val isLoading: Boolean = false,
    val isSyncing: Boolean = false,
    val walletCreated: Boolean = false,
    val walletAddress: String = "",
    val balance: Double = 0.0,
    val dailyDu: Double = 0.0,
    val error: String? = null
)

