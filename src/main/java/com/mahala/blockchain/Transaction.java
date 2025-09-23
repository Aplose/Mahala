package com.mahala.blockchain;

import java.math.BigDecimal;
import java.time.LocalDateTime;
import java.util.UUID;

public class Transaction {
    private final String id;
    private final String fromWalletId;
    private final String toWalletId;
    private final BigDecimal amount;
    private final LocalDateTime timestamp;
    private final String signature;
    private TransactionStatus status;

    public Transaction(String fromWalletId, String toWalletId, BigDecimal amount, String signature) {
        this.id = UUID.randomUUID().toString();
        this.fromWalletId = fromWalletId;
        this.toWalletId = toWalletId;
        this.amount = amount;
        this.timestamp = LocalDateTime.now();
        this.signature = signature;
        this.status = TransactionStatus.PENDING;
    }

    public String getId() { return id; }
    public String getFromWalletId() { return fromWalletId; }
    public String getToWalletId() { return toWalletId; }
    public BigDecimal getAmount() { return amount; }
    public LocalDateTime getTimestamp() { return timestamp; }
    public String getSignature() { return signature; }
    public TransactionStatus getStatus() { return status; }

    public void setStatus(TransactionStatus status) {
        this.status = status;
    }

    public enum TransactionStatus {
        PENDING,
        CONFIRMED,
        REJECTED
    }
}