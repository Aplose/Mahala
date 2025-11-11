# Mods Luanti (Minetest) pour Mahala

Mods pour intégrer Mahala dans les serveurs Luanti (Minetest).

## 📦 Mods disponibles

### 1. mahala_teleport
Système de téléportation payante entre mondes avec Mahala.

**Fonctionnalités :**
- Téléportation entre serveurs/mondes
- Calcul automatique du coût (distance + premium)
- Vérification de balance avant téléportation
- Transaction blockchain automatique

**Commandes :**
- `/teleport <world_name>` - Téléporter vers un monde
- `/teleport` - Afficher la liste des mondes
- `/wallet <adresse>` - Configurer votre wallet
- `/balance` - Vérifier votre balance

### 2. mahala_rental
Système de location d'emplacements dans les mondes.

**Fonctionnalités :**
- Création de zones locatives
- Location payante en Mahala
- Gestion automatique des contrats
- Expiration automatique

**Commandes :**
- `/create_rental <price>` - Créer une zone locative (admin)

### 3. mahala_nft
Affichage et interaction avec les NFT dans le monde 3D.

**Fonctionnalités :**
- Affichage de NFT en 3D
- Interaction avec les NFT
- Marketplace intégré
- Galeries NFT

**Commandes :**
- `/place_nft <nft_id>` - Placer un NFT dans le monde
- `/my_nfts` - Lister vos NFT

### 4. mahala_shop
Boutiques virtuelles liées à Dolibarr.

**Fonctionnalités :**
- Boutiques virtuelles dans le monde
- Synchronisation avec Dolibarr
- Achat de produits réels
- Gestion des stocks

**Commandes :**
- `/create_shop <name>` - Créer une boutique (admin)

## 🔧 Installation

### 1. Copier les mods

```bash
# Copier dans le répertoire mods de Minetest
cp -r luanti-mods/* /path/to/minetest/mods/
```

### 2. Configurer minetest.conf

```ini
# Connexion blockchain
mahala_node_url = http://node.mahala.org:8080
mahala_wallet_address = VOTRE_ADRESSE_WALLET

# Téléportation
teleport_base_cost = 1.0
teleport_distance_multiplier = 0.01

# NFT
nft_contract_address = 0x...

# Dolibarr
dolibarr_api_url = http://dolibarr.example.com/api
```

### 3. Activer les mods

Dans `world.mt` ou via l'interface :

```
load_mod_mahala_teleport = true
load_mod_mahala_rental = true
load_mod_mahala_nft = true
load_mod_mahala_shop = true
```

## 🎮 Utilisation

### Pour les joueurs

1. **Configurer son wallet** :
   ```
   /wallet abc123def456...
   ```

2. **Vérifier sa balance** :
   ```
   /balance
   ```

3. **Téléporter** :
   ```
   /teleport Hub Central
   ```

4. **Louer un emplacement** :
   - Cliquer sur un panneau "À louer"
   - Entrer la durée
   - Confirmer

5. **Acheter dans une boutique** :
   - Cliquer sur le panneau de la boutique
   - Choisir un produit
   - Confirmer l'achat

### Pour les administrateurs

1. **Créer une zone locative** :
   ```
   /create_rental 50
   ```
   (50 Mahala par jour)

2. **Créer une boutique** :
   ```
   /create_shop Ma Boutique
   ```

## 🔌 API requise

Les mods nécessitent que le Full Node Mahala soit accessible :

- Endpoint `/blockchain/balance/{address}` - Vérifier balance
- Endpoint `/transaction/submit` - Soumettre transaction

## 📝 Notes

- Les mods utilisent l'API HTTP de Minetest
- Les transactions sont synchrones (peuvent prendre quelques secondes)
- Les wallets sont stockés dans les métadonnées des joueurs
- Les locations et boutiques sont sauvegardées en JSON

## 🐛 Dépannage

### Erreur "HTTP API not available"
- Vérifier que `secure.enable_security = false` dans minetest.conf
- Ou activer l'API HTTP dans les paramètres

### Transactions échouent
- Vérifier que le nœud Mahala est accessible
- Vérifier l'URL dans minetest.conf
- Vérifier les logs du serveur

### Wallets non sauvegardés
- Vérifier les permissions d'écriture
- Vérifier le répertoire du monde

## 📄 Licence

MIT

