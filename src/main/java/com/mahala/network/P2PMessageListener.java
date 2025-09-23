package com.mahala.network;

public interface P2PMessageListener {
    void onMessage(String peerId, P2PMessage message);
}