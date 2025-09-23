package com.mahala.wallet;

public class BusinessWallet extends Wallet {
    private final String businessId;
    private final String businessName;
    private final String verificationDocument;
    private final String localIdentifier;

    public BusinessWallet(WalletType type, String businessId, String businessName,
                         String verificationDocument, String localIdentifier) {
        super(type);
        this.businessId = businessId;
        this.businessName = businessName;
        this.verificationDocument = verificationDocument;
        this.localIdentifier = localIdentifier;
    }

    @Override
    public boolean canReceiveTokens() {
        return true;
    }

    @Override
    public boolean canSendTokens() {
        return true;
    }

    public String getBusinessId() {
        return businessId;
    }

    public String getBusinessName() {
        return businessName;
    }

    public String getVerificationDocument() {
        return verificationDocument;
    }

    public String getLocalIdentifier() {
        return localIdentifier;
    }
}