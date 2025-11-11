package com.mahala

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.ui.Modifier
import com.mahala.ui.theme.MahalaTheme
import com.mahala.wallet.WalletScreen
import com.mahala.wallet.WalletViewModel

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        
        // Initialiser le light client Rust
        MahalaCore.initLightClient("http://node.mahala.org:8080")
        
        setContent {
            MahalaTheme {
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) {
                    val viewModel = WalletViewModel()
                    WalletScreen(viewModel = viewModel)
                }
            }
        }
    }
}

