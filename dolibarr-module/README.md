# Module Dolibarr Mahala

Module Dolibarr pour gérer les wallets Mahala et June, effectuer des échanges via le bridge, et intégrer avec le métaverse Luanti.

## 📋 Installation

### 1. Copier le module

```bash
# Copier le répertoire mahala dans custom/
cp -r dolibarr-module/mahala /path/to/dolibarr/custom/
```

### 2. Activer le module

1. Se connecter à Dolibarr en tant qu'administrateur
2. Aller dans **Home > Setup > Modules**
3. Chercher "Mahala" dans la liste
4. Cliquer sur **Activate**

### 3. Configurer

1. Aller dans **Home > Setup > Modules > Mahala**
2. Configurer :
   - **Mahala Node URL** : URL du nœud complet (ex: `http://node.mahala.org:8080`)
   - **Bridge URL** : URL du bridge (ex: `http://bridge.mahala.org:8081`)

## 🎯 Fonctionnalités

### Gestion des wallets

- Créer des wallets Mahala et June pour les tiers
- Synchroniser les balances automatiquement
- Afficher les wallets dans les fiches tiers

### Échanges via bridge

- Obtenir des devis pour échanger Mahala ↔ June
- Exécuter des échanges
- Historique des transactions

### Intégration métaverse

- Gérer les boutiques virtuelles
- Synchroniser avec Luanti
- Suivre les ventes dans le métaverse

## 📁 Structure

```
mahala/
├── core/
│   └── modules/
│       └── modMahala.class.php    # Module principal
├── class/
│   ├── wallet.class.php           # Gestion wallets
│   ├── exchange.class.php          # Gestion échanges
│   └── bridge_api.class.php       # API bridge
├── sql/
│   └── llx_mahala_wallet.sql      # Tables SQL
└── lib/
    └── mahala.lib.php             # Fonctions libres
```

## 🔧 Utilisation

### Créer un wallet pour un tiers

1. Ouvrir la fiche du tiers
2. Aller dans l'onglet "Mahala"
3. Cliquer sur "Créer wallet"
4. Choisir le type (Mahala ou June)
5. Entrer l'adresse du wallet

### Synchroniser une balance

1. Ouvrir le wallet
2. Cliquer sur "Synchroniser"
3. La balance sera mise à jour depuis la blockchain

### Effectuer un échange

1. Aller dans **Mahala > Échanges**
2. Choisir la direction (Mahala → June ou June → Mahala)
3. Entrer le montant
4. Voir le devis
5. Confirmer l'échange

## 📝 Tables SQL

Le module crée les tables suivantes :

- `llx_mahala_wallet` : Wallets des tiers
- `llx_mahala_transaction` : Transactions blockchain
- `llx_mahala_exchange` : Échanges via bridge
- `llx_mahala_virtual_shop` : Boutiques virtuelles

## 🔒 Permissions

Le module ajoute les permissions suivantes :

- `mahala` : Accès au module
- `mahala/read` : Lecture
- `mahala/write` : Écriture
- `mahala/admin` : Administration

## 🐛 Dépannage

### Le module n'apparaît pas

- Vérifier que le répertoire est dans `custom/mahala/`
- Vérifier les permissions des fichiers
- Vider le cache Dolibarr

### Erreur de connexion API

- Vérifier que le nœud et le bridge sont accessibles
- Vérifier les URLs dans la configuration
- Vérifier le firewall

## 📄 Licence

MIT

