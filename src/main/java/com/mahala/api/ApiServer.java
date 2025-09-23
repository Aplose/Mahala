package com.mahala.api;

import com.mahala.wallet.WalletManager;
import com.mahala.blockchain.TokenDistribution;
import com.mahala.consensus.RandomConsensus;
import io.javalin.Javalin;
import io.javalin.http.Context;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.Map;

public class ApiServer {
    private static final Logger logger = LoggerFactory.getLogger(ApiServer.class);

    private final Javalin app;
    private final WalletManager walletManager;
    private final TokenDistribution tokenDistribution;
    private final RandomConsensus consensus;
    private final int port;

    public ApiServer(int port, WalletManager walletManager,
                    TokenDistribution tokenDistribution, RandomConsensus consensus) {
        this.port = port;
        this.walletManager = walletManager;
        this.tokenDistribution = tokenDistribution;
        this.consensus = consensus;
        this.app = Javalin.create(config -> {
            config.http.defaultContentType = "application/json";
            config.bundledPlugins.enableCors(cors -> {
                cors.addRule(it -> {
                    it.anyHost();
                });
            });
        });

        setupRoutes();
    }

    private void setupRoutes() {
        app.before(this::authenticate);

        app.get("/api/health", this::getHealth);

        app.post("/api/wallets/personal", this::createPersonalWallet);
        app.post("/api/wallets/business", this::createBusinessWallet);
        app.get("/api/wallets/{walletId}", this::getWallet);
        app.get("/api/wallets/{walletId}/balance", this::getBalance);

        app.post("/api/transactions", this::createTransaction);
        app.get("/api/transactions/{transactionId}", this::getTransaction);

        app.get("/api/consensus/status", this::getConsensusStatus);
        app.get("/api/distribution/status", this::getDistributionStatus);

        app.exception(Exception.class, this::handleException);
    }

    private void authenticate(Context ctx) {
        String authHeader = ctx.header("Authorization");
        if (authHeader == null || !authHeader.startsWith("Bearer ")) {
            ctx.status(401).json(Map.of("error", "Missing or invalid authorization header"));
            return;
        }

        String token = authHeader.substring(7);
        if (!isValidToken(token)) {
            ctx.status(401).json(Map.of("error", "Invalid token"));
        }
    }

    private boolean isValidToken(String token) {
        return token != null && token.length() > 10;
    }

    private void getHealth(Context ctx) {
        ctx.json(Map.of(
            "status", "healthy",
            "timestamp", System.currentTimeMillis(),
            "activeNodes", consensus.getActiveNodeCount()
        ));
    }

    private void createPersonalWallet(Context ctx) {
        try {
            var request = ctx.bodyAsClass(CreatePersonalWalletRequest.class);
            var wallet = walletManager.createPersonalWallet(
                request.biometricData(), request.deviceId());

            ctx.status(201).json(Map.of(
                "walletId", wallet.getId(),
                "type", wallet.getType().toString(),
                "createdAt", wallet.getCreatedAt().toString()
            ));
        } catch (Exception e) {
            ctx.status(400).json(Map.of("error", e.getMessage()));
        }
    }

    private void createBusinessWallet(Context ctx) {
        try {
            var request = ctx.bodyAsClass(CreateBusinessWalletRequest.class);
            var wallet = walletManager.createBusinessWallet(
                request.type(), request.businessId(), request.businessName(),
                request.verificationDocument(), request.localIdentifier());

            ctx.status(201).json(Map.of(
                "walletId", wallet.getId(),
                "type", wallet.getType().toString(),
                "businessName", wallet.getBusinessName(),
                "createdAt", wallet.getCreatedAt().toString()
            ));
        } catch (Exception e) {
            ctx.status(400).json(Map.of("error", e.getMessage()));
        }
    }

    private void getWallet(Context ctx) {
        String walletId = ctx.pathParam("walletId");
        var wallet = walletManager.getWallet(walletId);

        if (wallet == null) {
            ctx.status(404).json(Map.of("error", "Wallet not found"));
            return;
        }

        ctx.json(Map.of(
            "walletId", wallet.getId(),
            "type", wallet.getType().toString(),
            "balance", wallet.getBalance().toString(),
            "active", wallet.isActive(),
            "createdAt", wallet.getCreatedAt().toString()
        ));
    }

    private void getBalance(Context ctx) {
        String walletId = ctx.pathParam("walletId");
        var wallet = walletManager.getWallet(walletId);

        if (wallet == null) {
            ctx.status(404).json(Map.of("error", "Wallet not found"));
            return;
        }

        ctx.json(Map.of(
            "balance", wallet.getBalance().toString(),
            "walletId", wallet.getId()
        ));
    }

    private void createTransaction(Context ctx) {
        ctx.status(501).json(Map.of("error", "Transaction creation not implemented yet"));
    }

    private void getTransaction(Context ctx) {
        ctx.status(501).json(Map.of("error", "Transaction retrieval not implemented yet"));
    }

    private void getConsensusStatus(Context ctx) {
        ctx.json(Map.of(
            "activeNodes", consensus.getActiveNodeCount(),
            "status", "active"
        ));
    }

    private void getDistributionStatus(Context ctx) {
        ctx.json(Map.of(
            "lastDistribution", tokenDistribution.getLastDistributionDate().toString(),
            "dailyAmount", tokenDistribution.getDailyTokenAmount().toString(),
            "distributedToday", tokenDistribution.hasDistributedToday()
        ));
    }

    private void handleException(Exception e, Context ctx) {
        logger.error("API error", e);
        ctx.status(500).json(Map.of("error", "Internal server error"));
    }

    public void start() {
        app.start(port);
        logger.info("API server started on port {}", port);
    }

    public void stop() {
        app.stop();
        logger.info("API server stopped");
    }
}