package com.mahala.wallet;

import java.math.BigDecimal;
import java.time.LocalDateTime;
import java.util.UUID;

public abstract class Wallet {
    protected final String id;
    protected final WalletType type;
    protected BigDecimal balance;
    protected final LocalDateTime createdAt;
    protected boolean active;

    public Wallet(WalletType type) {
        this.id = UUID.randomUUID().toString();
        this.type = type;
        this.balance = BigDecimal.ZERO;
        this.createdAt = LocalDateTime.now();
        this.active = true;
    }

    public String getId() {
        return id;
    }

    public WalletType getType() {
        return type;
    }

    public BigDecimal getBalance() {
        return balance;
    }

    public LocalDateTime getCreatedAt() {
        return createdAt;
    }

    public boolean isActive() {
        return active;
    }

    public void setActive(boolean active) {
        this.active = active;
    }

    public abstract boolean canReceiveTokens();

    public abstract boolean canSendTokens();

    public void addTokens(BigDecimal amount) {
        if (amount.compareTo(BigDecimal.ZERO) > 0) {
            this.balance = this.balance.add(amount);
        }
    }

    public boolean removeTokens(BigDecimal amount) {
        if (amount.compareTo(BigDecimal.ZERO) > 0 &&
            this.balance.compareTo(amount) >= 0 &&
            canSendTokens()) {
            this.balance = this.balance.subtract(amount);
            return true;
        }
        return false;
    }
}