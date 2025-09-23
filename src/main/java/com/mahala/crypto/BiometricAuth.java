package com.mahala.crypto;

import org.bouncycastle.crypto.digests.SHA256Digest;
import org.bouncycastle.util.encoders.Hex;

import java.security.SecureRandom;

public class BiometricAuth {
    private final SecureRandom random;
    private final SHA256Digest digest;

    public BiometricAuth() {
        this.random = new SecureRandom();
        this.digest = new SHA256Digest();
    }

    public String hashBiometricData(String biometricData, String salt) {
        String combined = biometricData + salt;
        byte[] input = combined.getBytes();

        digest.reset();
        digest.update(input, 0, input.length);

        byte[] result = new byte[digest.getDigestSize()];
        digest.doFinal(result, 0);

        return Hex.toHexString(result);
    }

    public String generateSalt() {
        byte[] salt = new byte[32];
        random.nextBytes(salt);
        return Hex.toHexString(salt);
    }

    public boolean verify(String biometricData, String storedHash) {
        String[] parts = storedHash.split(":");
        if (parts.length != 2) {
            return false;
        }

        String salt = parts[0];
        String hash = parts[1];

        String computedHash = hashBiometricData(biometricData, salt);
        return computedHash.equals(hash);
    }

    public String createSecureBiometricHash(String biometricData) {
        String salt = generateSalt();
        String hash = hashBiometricData(biometricData, salt);
        return salt + ":" + hash;
    }
}