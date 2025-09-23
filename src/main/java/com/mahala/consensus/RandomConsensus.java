package com.mahala.consensus;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.security.SecureRandom;
import java.util.List;
import java.util.ArrayList;
import java.util.Collections;
import java.util.concurrent.ConcurrentHashMap;
import java.util.Map;

public class RandomConsensus {
    private static final Logger logger = LoggerFactory.getLogger(RandomConsensus.class);

    private final SecureRandom random;
    private final Map<String, ConsensusNode> activeNodes;
    private final int minimumNodes;

    public RandomConsensus(int minimumNodes) {
        this.random = new SecureRandom();
        this.activeNodes = new ConcurrentHashMap<>();
        this.minimumNodes = minimumNodes;
    }

    public void registerNode(String nodeId, String publicKey) {
        ConsensusNode node = new ConsensusNode(nodeId, publicKey);
        activeNodes.put(nodeId, node);
        logger.info("Node registered for consensus: {}", nodeId);
    }

    public void unregisterNode(String nodeId) {
        activeNodes.remove(nodeId);
        logger.info("Node unregistered from consensus: {}", nodeId);
    }

    public List<String> selectRandomValidators(int count) {
        List<String> nodeIds = new ArrayList<>(activeNodes.keySet());

        if (nodeIds.size() < minimumNodes) {
            throw new IllegalStateException("Insufficient nodes for consensus. Required: "
                + minimumNodes + ", Available: " + nodeIds.size());
        }

        Collections.shuffle(nodeIds, random);
        return nodeIds.subList(0, Math.min(count, nodeIds.size()));
    }

    public String selectRandomLeader() {
        List<String> validators = selectRandomValidators(1);
        return validators.isEmpty() ? null : validators.get(0);
    }

    public boolean validateTransaction(String transactionId, String proposerNodeId) {
        if (!activeNodes.containsKey(proposerNodeId)) {
            logger.warn("Unknown proposer node: {}", proposerNodeId);
            return false;
        }

        int validatorCount = Math.min(5, activeNodes.size());
        List<String> validators = selectRandomValidators(validatorCount);

        logger.info("Transaction {} proposed by {}, validators: {}",
                   transactionId, proposerNodeId, validators);

        return validators.size() >= minimumNodes;
    }

    public ConsensusResult reachConsensus(String blockHash, List<String> transactions) {
        String leader = selectRandomLeader();
        if (leader == null) {
            return new ConsensusResult(false, null, "No leader available");
        }

        int validatorCount = Math.min(7, activeNodes.size());
        List<String> validators = selectRandomValidators(validatorCount);

        logger.info("Consensus round - Leader: {}, Validators: {}, Block: {}",
                   leader, validators, blockHash);

        boolean consensus = validators.size() >= minimumNodes;
        String result = consensus ? "ACCEPTED" : "REJECTED";

        return new ConsensusResult(consensus, leader, result);
    }

    public int getActiveNodeCount() {
        return activeNodes.size();
    }

    public static class ConsensusResult {
        private final boolean success;
        private final String leader;
        private final String result;

        public ConsensusResult(boolean success, String leader, String result) {
            this.success = success;
            this.leader = leader;
            this.result = result;
        }

        public boolean isSuccess() { return success; }
        public String getLeader() { return leader; }
        public String getResult() { return result; }
    }

    private static class ConsensusNode {
        private final String nodeId;
        private final String publicKey;

        public ConsensusNode(String nodeId, String publicKey) {
            this.nodeId = nodeId;
            this.publicKey = publicKey;
        }

        public String getNodeId() { return nodeId; }
        public String getPublicKey() { return publicKey; }
    }
}