# Guide de déploiement Mahala

## Prérequis

- Rust toolchain (stable)
- OpenSSL (pour certaines dépendances)
- PostgreSQL ou MySQL (pour Dolibarr)
- Dolibarr (pour le module)

## Déploiement Full Node

### 1. Compiler

```bash
cd full-node
cargo build --release
```

### 2. Configurer

Éditer `config.toml` :

```toml
api_port = 8080
p2p_port = 9000
bind_address = "0.0.0.0"
data_dir = "/var/lib/mahala/data"
```

### 3. Déployer avec systemd

```bash
./scripts/deploy-node.sh full-node mahala-node
```

### 4. Vérifier

```bash
sudo systemctl status mahala-node
curl http://localhost:8080/health
```

## Déploiement Bridge

### 1. Compiler

```bash
cd bridge
cargo build --release
```

### 2. Configurer

Éditer `config.toml` :

```toml
port = 8081
initial_june_reserve = 10000.0
initial_mahala_reserve = 10000.0
daily_limit = 1000.0
monthly_limit = 5000.0
```

### 3. Déployer

```bash
./scripts/deploy-node.sh bridge mahala-bridge
```

## Déploiement avec Docker

### Dockerfile Full Node

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin mahala-node

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/mahala-node /usr/local/bin/
EXPOSE 8080
CMD ["mahala-node"]
```

### Docker Compose

```yaml
version: '3.8'

services:
  full-node:
    build: ./full-node
    ports:
      - "8080:8080"
    volumes:
      - ./data:/app/data
    environment:
      - RUST_LOG=info

  bridge:
    build: ./bridge
    ports:
      - "8081:8081"
    depends_on:
      - full-node
    environment:
      - MAHALA_NODE_URL=http://full-node:8080
```

## Déploiement mobile

### Android

Voir `mobile/android/README.md`

### iOS

Voir `mobile/ios/README.md` et `mobile/ios/BUILD_INSTRUCTIONS.md`

## Module Dolibarr

### 1. Installer

```bash
cp -r dolibarr-module/mahala /path/to/dolibarr/custom/
```

### 2. Activer

1. Dolibarr > Setup > Modules
2. Activer "Mahala"

### 3. Configurer

1. Dolibarr > Setup > Modules > Mahala
2. Configurer les URLs :
   - Mahala Node URL
   - Bridge URL

## Monitoring

### Logs

```bash
# Full Node
sudo journalctl -u mahala-node -f

# Bridge
sudo journalctl -u mahala-bridge -f
```

### Métriques

- Health check : `GET /health`
- Statistiques bridge : `GET /bridge/stats`
- Taille mempool : `GET /mempool/size`

## Sécurité

### Firewall

```bash
# Autoriser uniquement les ports nécessaires
sudo ufw allow 8080/tcp  # Full Node API
sudo ufw allow 8081/tcp  # Bridge API
sudo ufw allow 9000/tcp  # P2P (si exposé)
```

### Reverse Proxy (Nginx)

```nginx
server {
    listen 80;
    server_name node.mahala.org;

    location / {
        proxy_pass http://localhost:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

## Backup

### Blockchain

```bash
# Sauvegarder les données
tar -czf mahala-backup-$(date +%Y%m%d).tar.gz /var/lib/mahala/data
```

### Base de données Dolibarr

```bash
mysqldump -u user -p dolibarr > dolibarr-backup.sql
```

## Mise à jour

### Full Node

```bash
cd full-node
git pull
cargo build --release
sudo systemctl restart mahala-node
```

### Bridge

```bash
cd bridge
git pull
cargo build --release
sudo systemctl restart mahala-bridge
```

## Dépannage

### Le nœud ne démarre pas

1. Vérifier les logs : `sudo journalctl -u mahala-node`
2. Vérifier les permissions : `ls -la /var/lib/mahala/data`
3. Vérifier le port : `netstat -tulpn | grep 8080`

### Erreur de connexion API

1. Vérifier que le service tourne
2. Vérifier le firewall
3. Vérifier les logs

### Problèmes de synchronisation

1. Vérifier la connectivité réseau
2. Vérifier les bootstrap nodes
3. Vérifier les logs P2P

