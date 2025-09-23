package com.mahala.api;

import com.mahala.wallet.WalletType;

public record CreateBusinessWalletRequest(
    WalletType type,
    String businessId,
    String businessName,
    String verificationDocument,
    String localIdentifier
) {
}