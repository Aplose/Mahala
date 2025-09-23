package com.mahala;

import com.mahala.api.ApiServer;
import com.mahala.blockchain.TokenDistribution;
import com.mahala.consensus.RandomConsensus;
import com.mahala.network.P2PNode;
import com.mahala.network.P2PMessage;
import com.mahala.network.P2PMessageListener;
import com.mahala.wallet.WalletManager;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.Arrays;
import java.util.List;
import java.util.UUID;

public class MahalaNode implements P2PMessageListener {
    private static final Logger logger = LoggerFactory.getLogger(MahalaNode.class);

    private final String nodeId;
    private final WalletManager walletManager;
    private final RandomConsensus consensus;
    private final TokenDistribution tokenDistribution;
    private final P2PNode p2pNode;
    private final ApiServer apiServer;

    public MahalaNode() {
        this.nodeId = "mahala-" + UUID.randomUUID().toString().substring(0, 8);

        this.walletManager = new WalletManager();
        this.consensus = new RandomConsensus(3);
        this.tokenDistribution = new TokenDistribution(walletManager);

        List<String> seedNodes = Arrays.asList(
            "127.0.0.1:8001",
            "127.0.0.1:8002"
        );

        int p2pPort = getP2PPort();
        int apiPort = getApiPort();

        this.p2pNode = new P2PNode(nodeId, p2pPort, seedNodes);
        this.apiServer = new ApiServer(apiPort, walletManager, tokenDistribution, consensus);

        setupMessageHandling();
    }

    public static void main(String[] args) {
        logger.info("Starting Mahala Cryptocurrency Node...");

        MahalaNode node = new MahalaNode();

        Runtime.getRuntime().addShutdownHook(new Thread(() -> {
            logger.info("Shutting down Mahala Node...");
            node.stop();
        }));

        node.start();

        try {
            Thread.currentThread().join();
        } catch (InterruptedException e) {
            logger.warn("Main thread interrupted", e);
            Thread.currentThread().interrupt();
        }
    }

    public void start() {
        logger.info("Starting Mahala Node: {}", nodeId);

        consensus.registerNode(nodeId, "public-key-placeholder");

        p2pNode.start();

        apiServer.start();

        tokenDistribution.startDailyDistribution();

        logger.info("Mahala Node {} initialized successfully", nodeId);
        logger.info("P2P listening on port: {}", p2pNode.getNodeId());
        logger.info("API server running on port: 8080");
        logger.info("Ready to accept connections");
    }

    public void stop() {
        logger.info("Stopping Mahala Node: {}", nodeId);

        tokenDistribution.stopDailyDistribution();
        apiServer.stop();
        p2pNode.stop();

        consensus.unregisterNode(nodeId);

        logger.info("Mahala Node {} stopped", nodeId);
    }

    private void setupMessageHandling() {
        p2pNode.addMessageListener(this);
    }

    @Override
    public void onMessage(String peerId, P2PMessage message) {
        logger.debug("Received {} message from {}: {}", message.getType(), peerId, message.getPayload());

        switch (message.getType()) {
            case "HANDSHAKE":
                handleHandshake(peerId, message);
                break;
            case "TRANSACTION":
                handleTransaction(peerId, message);
                break;
            case "CONSENSUS":
                handleConsensus(peerId, message);
                break;
            default:
                logger.warn("Unknown message type: {}", message.getType());
        }
    }

    private void handleHandshake(String peerId, P2PMessage message) {
        logger.info("Handshake received from {}: {}", peerId, message.getPayload());

        P2PMessage response = new P2PMessage("HANDSHAKE_ACK", nodeId, "Hello from " + nodeId);
        p2pNode.sendMessage(p2pNode.getChannelForPeer(peerId), response);
    }

    private void handleTransaction(String peerId, P2PMessage message) {
        logger.info("Transaction received from {}", peerId);
    }

    private void handleConsensus(String peerId, P2PMessage message) {
        logger.info("Consensus message received from {}", peerId);
    }

    private int getP2PPort() {
        String portStr = System.getProperty("mahala.p2p.port", "8000");
        return Integer.parseInt(portStr);
    }

    private int getApiPort() {
        String portStr = System.getProperty("mahala.api.port", "8080");
        return Integer.parseInt(portStr);
    }

    public String getNodeId() {
        return nodeId;
    }
}