package com.mahala.network;

import io.netty.bootstrap.Bootstrap;
import io.netty.bootstrap.ServerBootstrap;
import io.netty.channel.*;
import io.netty.channel.nio.NioEventLoopGroup;
import io.netty.channel.socket.SocketChannel;
import io.netty.channel.socket.nio.NioServerSocketChannel;
import io.netty.channel.socket.nio.NioSocketChannel;
import io.netty.handler.codec.string.StringDecoder;
import io.netty.handler.codec.string.StringEncoder;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.net.InetSocketAddress;
import java.util.List;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.CopyOnWriteArrayList;

public class P2PNode {
    private static final Logger logger = LoggerFactory.getLogger(P2PNode.class);

    private final String nodeId;
    private final int port;
    private final List<String> seedNodes;
    private final ConcurrentHashMap<String, Channel> connectedPeers;
    private final List<P2PMessageListener> messageListeners;

    private EventLoopGroup bossGroup;
    private EventLoopGroup workerGroup;
    private Channel serverChannel;

    public P2PNode(String nodeId, int port, List<String> seedNodes) {
        this.nodeId = nodeId;
        this.port = port;
        this.seedNodes = new CopyOnWriteArrayList<>(seedNodes);
        this.connectedPeers = new ConcurrentHashMap<>();
        this.messageListeners = new CopyOnWriteArrayList<>();
    }

    public void start() {
        bossGroup = new NioEventLoopGroup(1);
        workerGroup = new NioEventLoopGroup();

        try {
            ServerBootstrap serverBootstrap = new ServerBootstrap();
            serverBootstrap.group(bossGroup, workerGroup)
                    .channel(NioServerSocketChannel.class)
                    .childHandler(new ChannelInitializer<SocketChannel>() {
                        @Override
                        protected void initChannel(SocketChannel ch) {
                            ch.pipeline().addLast(new StringDecoder());
                            ch.pipeline().addLast(new StringEncoder());
                            ch.pipeline().addLast(new P2PServerHandler(P2PNode.this));
                        }
                    })
                    .option(ChannelOption.SO_BACKLOG, 128)
                    .childOption(ChannelOption.SO_KEEPALIVE, true);

            ChannelFuture future = serverBootstrap.bind(port).sync();
            serverChannel = future.channel();

            logger.info("P2P Node {} started on port {}", nodeId, port);

            connectToSeedNodes();

        } catch (InterruptedException e) {
            logger.error("Failed to start P2P node", e);
            Thread.currentThread().interrupt();
        }
    }

    public void stop() {
        if (serverChannel != null) {
            serverChannel.close();
        }
        if (workerGroup != null) {
            workerGroup.shutdownGracefully();
        }
        if (bossGroup != null) {
            bossGroup.shutdownGracefully();
        }
        logger.info("P2P Node {} stopped", nodeId);
    }

    private void connectToSeedNodes() {
        for (String seedNode : seedNodes) {
            String[] parts = seedNode.split(":");
            if (parts.length == 2) {
                String host = parts[0];
                int seedPort = Integer.parseInt(parts[1]);
                connectToPeer(host, seedPort);
            }
        }
    }

    public void connectToPeer(String host, int peerPort) {
        if (peerPort == this.port && ("localhost".equals(host) || "127.0.0.1".equals(host))) {
            return;
        }

        Bootstrap bootstrap = new Bootstrap();
        bootstrap.group(workerGroup)
                .channel(NioSocketChannel.class)
                .handler(new ChannelInitializer<SocketChannel>() {
                    @Override
                    protected void initChannel(SocketChannel ch) {
                        ch.pipeline().addLast(new StringDecoder());
                        ch.pipeline().addLast(new StringEncoder());
                        ch.pipeline().addLast(new P2PClientHandler(P2PNode.this));
                    }
                });

        try {
            ChannelFuture future = bootstrap.connect(host, peerPort).sync();
            Channel channel = future.channel();
            String peerId = host + ":" + peerPort;
            connectedPeers.put(peerId, channel);

            sendMessage(channel, new P2PMessage("HANDSHAKE", nodeId, "Hello from " + nodeId));
            logger.info("Connected to peer: {}", peerId);

        } catch (Exception e) {
            logger.warn("Failed to connect to peer {}:{}", host, peerPort);
        }
    }

    public void addPeer(String peerId, Channel channel) {
        connectedPeers.put(peerId, channel);
        logger.info("Added peer: {}", peerId);
    }

    public void removePeer(String peerId) {
        Channel channel = connectedPeers.remove(peerId);
        if (channel != null) {
            channel.close();
        }
        logger.info("Removed peer: {}", peerId);
    }

    public void broadcastMessage(P2PMessage message) {
        for (Channel channel : connectedPeers.values()) {
            sendMessage(channel, message);
        }
    }

    public void sendMessage(Channel channel, P2PMessage message) {
        if (channel != null && channel.isActive()) {
            channel.writeAndFlush(message.toJson());
        }
    }

    public void addMessageListener(P2PMessageListener listener) {
        messageListeners.add(listener);
    }

    public void handleMessage(String peerId, P2PMessage message) {
        for (P2PMessageListener listener : messageListeners) {
            listener.onMessage(peerId, message);
        }
    }

    public String getNodeId() {
        return nodeId;
    }

    public int getConnectedPeerCount() {
        return connectedPeers.size();
    }

    public List<String> getConnectedPeers() {
        return List.copyOf(connectedPeers.keySet());
    }
}