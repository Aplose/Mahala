package com.mahala.wallet

import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.mahala.biometric.BiometricManager
import androidx.fragment.app.FragmentActivity

/**
 * Écran principal du wallet Mahala
 */
@Composable
fun WalletScreen(
    viewModel: WalletViewModel,
    activity: FragmentActivity? = null
) {
    val uiState by viewModel.uiState.collectAsState()

    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.spacedBy(16.dp)
    ) {
        // Titre
        Text(
            text = "Mahala Wallet",
            style = MaterialTheme.typography.headlineLarge,
            fontWeight = FontWeight.Bold
        )

        when {
            // Écran de création de wallet
            !uiState.walletCreated -> {
                CreateWalletScreen(
                    viewModel = viewModel,
                    activity = activity,
                    isLoading = uiState.isLoading
                )
            }
            
            // Écran du wallet
            else -> {
                WalletContent(
                    uiState = uiState,
                    onRefresh = {
                        viewModel.refreshBalance()
                        viewModel.refreshDailyDu()
                    },
                    onSync = { viewModel.sync() }
                )
            }
        }

        // Message d'erreur
        uiState.error?.let { error ->
            Card(
                modifier = Modifier.fillMaxWidth(),
                colors = CardDefaults.cardColors(
                    containerColor = MaterialTheme.colorScheme.errorContainer
                )
            ) {
                Text(
                    text = error,
                    modifier = Modifier.padding(16.dp),
                    color = MaterialTheme.colorScheme.onErrorContainer
                )
            }
        }
    }
}

/**
 * Écran de création de wallet
 */
@Composable
fun CreateWalletScreen(
    viewModel: WalletViewModel,
    activity: FragmentActivity?,
    isLoading: Boolean
) {
    Column(
        modifier = Modifier.fillMaxWidth(),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.spacedBy(16.dp)
    ) {
        Text(
            text = "Créer votre portefeuille membre",
            style = MaterialTheme.typography.titleLarge
        )

        Text(
            text = "Votre wallet sera créé de manière sécurisée à partir de votre biométrie. " +
                    "Les données biométriques ne sont jamais stockées.",
            style = MaterialTheme.typography.bodyMedium,
            modifier = Modifier.padding(horizontal = 16.dp)
        )

        Button(
            onClick = {
                activity?.let {
                    val biometricManager = BiometricManager(it)
                    if (biometricManager.isAvailable()) {
                        biometricManager.authenticate(
                            onSuccess = { hash ->
                                viewModel.createWallet(hash)
                            },
                            onError = { error ->
                                // Gérer l'erreur
                            }
                        )
                    }
                }
            },
            enabled = !isLoading && activity != null,
            modifier = Modifier.fillMaxWidth(0.8f)
        ) {
            if (isLoading) {
                CircularProgressIndicator(modifier = Modifier.size(20.dp))
                Spacer(modifier = Modifier.width(8.dp))
            }
            Text("Créer avec biométrie")
        }
    }
}

/**
 * Contenu du wallet
 */
@Composable
fun WalletContent(
    uiState: WalletUiState,
    onRefresh: () -> Unit,
    onSync: () -> Unit
) {
    Column(
        modifier = Modifier.fillMaxWidth(),
        verticalArrangement = Arrangement.spacedBy(16.dp)
    ) {
        // Balance
        Card(
            modifier = Modifier.fillMaxWidth(),
            colors = CardDefaults.cardColors(
                containerColor = MaterialTheme.colorScheme.primaryContainer
            )
        ) {
            Column(
                modifier = Modifier.padding(24.dp),
                horizontalAlignment = Alignment.CenterHorizontally
            ) {
                Text(
                    text = "Solde Mahala",
                    style = MaterialTheme.typography.titleMedium
                )
                Spacer(modifier = Modifier.height(8.dp))
                Text(
                    text = "💎 ${String.format("%.2f", uiState.balance)} M",
                    style = MaterialTheme.typography.headlineLarge,
                    fontWeight = FontWeight.Bold
                )
            }
        }

        // Dividende Universel
        Card(
            modifier = Modifier.fillMaxWidth(),
            colors = CardDefaults.cardColors(
                containerColor = MaterialTheme.colorScheme.secondaryContainer
            )
        ) {
            Column(
                modifier = Modifier.padding(24.dp)
            ) {
                Text(
                    text = "Dividende Universel",
                    style = MaterialTheme.typography.titleMedium
                )
                Spacer(modifier = Modifier.height(8.dp))
                Text(
                    text = "Aujourd'hui: +${String.format("%.4f", uiState.dailyDu)} M",
                    style = MaterialTheme.typography.bodyLarge,
                    color = MaterialTheme.colorScheme.primary
                )
                Text(
                    text = "L'app doit tourner en arrière-plan pour recevoir le DU",
                    style = MaterialTheme.typography.bodySmall,
                    modifier = Modifier.padding(top = 8.dp)
                )
            }
        }

        // Adresse du wallet
        Card(
            modifier = Modifier.fillMaxWidth()
        ) {
            Column(
                modifier = Modifier.padding(16.dp)
            ) {
                Text(
                    text = "Adresse du wallet",
                    style = MaterialTheme.typography.titleSmall
                )
                Spacer(modifier = Modifier.height(4.dp))
                Text(
                    text = uiState.walletAddress.take(20) + "...",
                    style = MaterialTheme.typography.bodySmall,
                    fontFamily = androidx.compose.ui.text.font.FontFamily.Monospace
                )
            }
        }

        // Boutons d'action
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.spacedBy(8.dp)
        ) {
            Button(
                onClick = onRefresh,
                modifier = Modifier.weight(1f)
            ) {
                Text("Actualiser")
            }

            Button(
                onClick = onSync,
                enabled = !uiState.isSyncing,
                modifier = Modifier.weight(1f)
            ) {
                if (uiState.isSyncing) {
                    CircularProgressIndicator(modifier = Modifier.size(16.dp))
                } else {
                    Text("Synchroniser")
                }
            }
        }
    }
}

