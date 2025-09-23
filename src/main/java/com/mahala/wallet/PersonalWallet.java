package com.mahala.wallet;

import com.mahala.crypto.BiometricAuth;

public class PersonalWallet extends Wallet {
    private final String biometricHash;
    private final String deviceId;
    private final BiometricAuth biometricAuth;

    public PersonalWallet(String biometricHash, String deviceId) {
        super(WalletType.PERSONAL);
        this.biometricHash = biometricHash;
        this.deviceId = deviceId;
        this.biometricAuth = new BiometricAuth();
    }

    @Override
    public boolean canReceiveTokens() {
        return true;
    }

    @Override
    public boolean canSendTokens() {
        return true;
    }

    public boolean authenticateBiometric(String biometricData) {
        return biometricAuth.verify(biometricData, biometricHash);
    }

    public String getBiometricHash() {
        return biometricHash;
    }

    public String getDeviceId() {
        return deviceId;
    }
}