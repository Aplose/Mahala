package com.mahala.wallet;

import com.mahala.crypto.BiometricAuth;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.math.BigDecimal;
import java.util.concurrent.ConcurrentHashMap;
import java.util.Map;
import java.util.List;
import java.util.ArrayList;
import java.util.stream.Collectors;

public class WalletManager {
    private static final Logger logger = LoggerFactory.getLogger(WalletManager.class);

    private final Map<String, Wallet> wallets;
    private final Map<String, String> biometricToWalletId;
    private final Map<String, String> businessIdToWalletId;
    private final BiometricAuth biometricAuth;

    public WalletManager() {
        this.wallets = new ConcurrentHashMap<>();
        this.biometricToWalletId = new ConcurrentHashMap<>();
        this.businessIdToWalletId = new ConcurrentHashMap<>();
        this.biometricAuth = new BiometricAuth();
    }

    public PersonalWallet createPersonalWallet(String biometricData, String deviceId) {
        if (biometricToWalletId.containsKey(biometricData)) {
            throw new IllegalArgumentException("Wallet already exists for this biometric data");
        }

        String biometricHash = biometricAuth.createSecureBiometricHash(biometricData);
        PersonalWallet wallet = new PersonalWallet(biometricHash, deviceId);

        wallets.put(wallet.getId(), wallet);
        biometricToWalletId.put(biometricData, wallet.getId());

        logger.info("Created personal wallet: {}", wallet.getId());
        return wallet;
    }

    public BusinessWallet createBusinessWallet(WalletType type, String businessId,
                                             String businessName, String verificationDocument,
                                             String localIdentifier) {
        if (businessIdToWalletId.containsKey(businessId)) {
            throw new IllegalArgumentException("Wallet already exists for this business ID");
        }

        BusinessWallet wallet = new BusinessWallet(type, businessId, businessName,
                                                 verificationDocument, localIdentifier);

        wallets.put(wallet.getId(), wallet);
        businessIdToWalletId.put(businessId, wallet.getId());

        logger.info("Created business wallet: {} for {}", wallet.getId(), businessName);
        return wallet;
    }

    public Wallet getWallet(String walletId) {
        return wallets.get(walletId);
    }

    public List<PersonalWallet> getAllPersonalWallets() {
        return wallets.values().stream()
                .filter(w -> w.getType() == WalletType.PERSONAL)
                .map(w -> (PersonalWallet) w)
                .filter(Wallet::isActive)
                .collect(Collectors.toList());
    }

    public List<BusinessWallet> getAllBusinessWallets() {
        return wallets.values().stream()
                .filter(w -> w.getType() != WalletType.PERSONAL)
                .map(w -> (BusinessWallet) w)
                .filter(Wallet::isActive)
                .collect(Collectors.toList());
    }

    public boolean authenticatePersonalWallet(String walletId, String biometricData) {
        Wallet wallet = wallets.get(walletId);
        if (wallet instanceof PersonalWallet) {
            return ((PersonalWallet) wallet).authenticateBiometric(biometricData);
        }
        return false;
    }

    public boolean transferTokens(String fromWalletId, String toWalletId, BigDecimal amount) {
        Wallet fromWallet = wallets.get(fromWalletId);
        Wallet toWallet = wallets.get(toWalletId);

        if (fromWallet == null || toWallet == null) {
            return false;
        }

        if (fromWallet.removeTokens(amount)) {
            toWallet.addTokens(amount);
            logger.info("Transferred {} tokens from {} to {}", amount, fromWalletId, toWalletId);
            return true;
        }

        return false;
    }
}