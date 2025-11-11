# Installation des mods Luanti Mahala

## Prérequis

- Minetest 5.0+ ou Luanti
- Serveur Mahala accessible (Full Node)
- Permissions d'écriture dans le répertoire du monde

## Installation

### Méthode 1 : Installation manuelle

1. **Télécharger les mods** :
   ```bash
   cd /path/to/minetest/mods
   git clone <repo> luanti-mods
   ```

2. **Copier les mods** :
   ```bash
   cp -r luanti-mods/mahala_teleport .
   cp -r luanti-mods/mahala_rental .
   cp -r luanti-mods/mahala_nft .
   cp -r luanti-mods/mahala_shop .
   ```

3. **Configurer** :
   Éditer `minetest.conf` (voir `config_example.conf`)

4. **Activer les mods** :
   Dans `world.mt` :
   ```
   load_mod_mahala_teleport = true
   load_mod_mahala_rental = true
   load_mod_mahala_nft = true
   load_mod_mahala_shop = true
   ```

### Méthode 2 : Via ContentDB (à venir)

Les mods seront disponibles sur ContentDB pour installation automatique.

## Configuration

### 1. Wallet du serveur

Le serveur doit avoir un wallet Mahala pour recevoir les paiements :

```bash
# Générer un wallet (utiliser l'app mobile ou un outil)
# Puis configurer dans minetest.conf :
mahala_wallet_address = abc123def456...
```

### 2. URL du nœud

```ini
mahala_node_url = http://node.mahala.org:8080
```

### 3. Coûts de téléportation

```ini
teleport_base_cost = 1.0          # Coût de base (Mahala)
teleport_distance_multiplier = 0.01  # Multiplicateur par distance
```

## Vérification

1. Démarrer le serveur
2. Vérifier les logs pour :
   ```
   [Mahala Teleport] Mod chargé
   [Mahala Rental] Mod chargé
   [Mahala NFT] Mod chargé
   [Mahala Shop] Mod chargé
   ```

3. Tester avec un joueur :
   ```
   /wallet <adresse>
   /balance
   /teleport
   ```

## Dépannage

### Les mods ne se chargent pas

- Vérifier les noms des répertoires (doivent correspondre aux noms des mods)
- Vérifier `world.mt` pour les `load_mod_*`
- Vérifier les logs d'erreur

### Erreurs HTTP

- Vérifier que `secure.enable_security = false` ou que les mods sont dans `trusted_mods`
- Vérifier la connectivité réseau vers le nœud Mahala
- Vérifier l'URL dans la config

### Transactions échouent

- Vérifier que le wallet du serveur est configuré
- Vérifier que le nœud Mahala est accessible
- Vérifier les logs du nœud

## Support

Pour les problèmes et questions :
- Issues GitHub
- Forum Mahala
- Discord/Matrix

