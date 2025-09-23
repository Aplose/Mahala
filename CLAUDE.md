# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Java-based cryptocurrency node project called "Mahala" built with Gradle. The project implements a blockchain node with modules for cryptocurrency functionality including blockchain operations, cryptography, networking, storage, wallet management, consensus mechanisms, and API endpoints.

## Core Project Rules and Objectives

### Wallet Management
- **Personal wallets (one per human user)**: Each personal wallet must be tied to a unique human user authenticated through biometric verification on their mobile device
- **Biometric authentication**: Personal wallet access requires biometric verification (fingerprint, face recognition, etc.) on the user's phone
- **No shared personal wallets**: Personal wallets cannot be shared between multiple users
- **Business/Organization wallets**: Simple deposit-only wallets limited to one per recognized entity:
  - **Enterprises**: One wallet per company verified through local professional identifiers (SIRET, business license, etc.)
  - **Associations**: One wallet per registered association verified through official registration documents
  - **Organizations**: One wallet per recognized organization verified through appropriate local credentials
- **Business wallet restrictions**: Business/organization wallets do NOT generate tokens and are deposit-only

### Token Distribution
- **Daily equal distribution**: Mahala tokens must be distributed equally to all personal wallets every day
- **Fair allocation**: Every participating personal wallet receives the same amount of tokens daily
- **Personal wallets only**: Only biometrically-verified personal wallets are eligible for token generation
- **Business wallets excluded**: Business/organization wallets do NOT receive daily token distribution
- **Automated distribution**: The distribution process should be automated and transparent

### Consensus Mechanism
- **Non-competitive consensus**: The consensus process must not involve competition between nodes (no "mining race")
- **Randomness-based**: Consensus decisions should be based on randomness rather than computational power
- **No proof-of-work**: Avoid Bitcoin-style proof-of-work that leads to energy waste and centralization
- **Participation-based**: All network participants can contribute to consensus through their nodes

### Network Participation
- **Every participant runs a node**: Each network participant operates their own validation node
- **Transaction validation**: All nodes participate in validating transactions
- **Decentralized validation**: No single entity controls transaction validation
- **Equal participation**: All nodes have equal weight in the validation process
- **Automatic peer-to-peer mesh**: All nodes must automatically discover and connect to each other in a peer-to-peer mesh network
- **Server deployment**: Nodes can be deployed on servers to provide API access for client applications

### API and Client Access
- **Secure REST API**: A complete, secure REST API must be provided for client applications
- **Full API coverage**: The API must expose all necessary functionality for wallet management, transactions, and network operations
- **Client program support**: External client programs must be able to interact with the network through the API
- **Server nodes**: Dedicated server nodes provide API endpoints for client applications that don't run their own nodes
- **API security**: All API endpoints must implement proper authentication and authorization mechanisms

## Build and Development Commands

### Build
- `./gradlew build` - Build the entire project
- `./gradlew compileJava` - Compile Java sources only
- `./gradlew jar` - Create JAR file

### Running
- `./gradlew run` - Run the application (main class: com.mahala.MahalaNode)
- `java -jar build/libs/mahala.jar` - Run the built JAR

### Testing
- `./gradlew test` - Run all tests with JUnit Jupiter
- `./gradlew test --tests ClassName` - Run specific test class
- `./gradlew test --tests ClassName.methodName` - Run specific test method

### Other Gradle Tasks
- `./gradlew clean` - Clean build artifacts
- `./gradlew check` - Run all checks (tests, etc.)
- `./gradlew tasks` - List all available tasks

## Architecture

The codebase follows a modular architecture with clear separation of concerns:

- **Main Entry Point**: `com.mahala.MahalaNode` - The main application class
- **Package Structure**:
  - `com.mahala.blockchain` - Blockchain core functionality
  - `com.mahala.crypto` - Cryptographic operations (uses BouncyCastle)
  - `com.mahala.network` - Network communication and peer-to-peer mesh functionality
  - `com.mahala.storage` - Data persistence (uses H2 database)
  - `com.mahala.wallet` - Wallet management (personal and business wallets)
  - `com.mahala.consensus` - Non-competitive randomness-based consensus algorithm
  - `com.mahala.api` - Secure REST API endpoints for client applications
  - `com.mahala.util` - Utility classes and helper functions

## Key Dependencies

- **H2 Database** (v2.2.224) - Embedded database for storage
- **Jackson** (v2.16.0) - JSON processing
- **BouncyCastle** (v1.77) - Cryptographic operations
- **SLF4J + Logback** - Logging framework
- **JUnit Jupiter** (v5.10.1) - Testing framework

## Java Configuration

- **Java Version**: 21 (configured via toolchain)
- **Application Plugin**: Configured with main class `com.mahala.MahalaNode`
- **Gradle JVM Args**: `-Xmx512m -XX:+UseSerialGC` with daemon disabled for resource-constrained environments