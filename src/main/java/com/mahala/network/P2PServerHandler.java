package com.mahala.network;

import io.netty.channel.ChannelHandlerContext;
import io.netty.channel.ChannelInboundHandlerAdapter;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class P2PServerHandler extends ChannelInboundHandlerAdapter {
    private static final Logger logger = LoggerFactory.getLogger(P2PServerHandler.class);

    private final P2PNode node;

    public P2PServerHandler(P2PNode node) {
        this.node = node;
    }

    @Override
    public void channelActive(ChannelHandlerContext ctx) {
        String peerId = ctx.channel().remoteAddress().toString();
        node.addPeer(peerId, ctx.channel());
        logger.info("New peer connected: {}", peerId);
    }

    @Override
    public void channelInactive(ChannelHandlerContext ctx) {
        String peerId = ctx.channel().remoteAddress().toString();
        node.removePeer(peerId);
        logger.info("Peer disconnected: {}", peerId);
    }

    @Override
    public void channelRead(ChannelHandlerContext ctx, Object msg) {
        try {
            String messageJson = (String) msg;
            P2PMessage message = P2PMessage.fromJson(messageJson);
            String peerId = ctx.channel().remoteAddress().toString();

            logger.debug("Received message from {}: {}", peerId, message.getType());
            node.handleMessage(peerId, message);
        } catch (Exception e) {
            logger.error("Error processing message", e);
        }
    }

    @Override
    public void exceptionCaught(ChannelHandlerContext ctx, Throwable cause) {
        logger.error("Error in P2P server handler", cause);
        ctx.close();
    }
}