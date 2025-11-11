#!/bin/bash
# Script de déploiement d'un nœud Mahala

set -e

NODE_TYPE=$1
SERVICE_NAME=${2:-mahala-node}

if [ -z "$NODE_TYPE" ]; then
    echo "Usage: $0 [full-node|bridge] [service-name]"
    exit 1
fi

cd "$(dirname "$0")/.."

echo "🚀 Déploiement du $NODE_TYPE..."

# Compiler
echo "🔨 Compilation..."
cd $NODE_TYPE
cargo build --release

# Créer le répertoire d'installation
INSTALL_DIR="/usr/local/bin"
echo "📦 Installation dans $INSTALL_DIR..."

sudo cp target/release/mahala-$NODE_TYPE $INSTALL_DIR/mahala-$NODE_TYPE
sudo chmod +x $INSTALL_DIR/mahala-$NODE_TYPE

# Créer le service systemd
echo "⚙️  Configuration du service systemd..."

SERVICE_FILE="/etc/systemd/system/$SERVICE_NAME.service"

sudo tee $SERVICE_FILE > /dev/null <<EOF
[Unit]
Description=Mahala $NODE_TYPE
After=network.target

[Service]
Type=simple
User=$USER
WorkingDirectory=$(pwd)
ExecStart=$INSTALL_DIR/mahala-$NODE_TYPE
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
EOF

# Recharger systemd
sudo systemctl daemon-reload

# Activer et démarrer le service
echo "▶️  Démarrage du service..."
sudo systemctl enable $SERVICE_NAME
sudo systemctl start $SERVICE_NAME

# Afficher le statut
echo "📊 Statut du service:"
sudo systemctl status $SERVICE_NAME --no-pager

echo "✅ Déploiement terminé!"
echo ""
echo "Commandes utiles:"
echo "  sudo systemctl status $SERVICE_NAME"
echo "  sudo systemctl restart $SERVICE_NAME"
echo "  sudo journalctl -u $SERVICE_NAME -f"

