# Mahala Métaverse - Intégration Luanti & Dolibarr

## 🌐 Vision : L'économie réelle dans le monde virtuel

Mahala combine blockchain, monde virtuel Luanti et commerce réel via Dolibarr pour créer un écosystème économique complet où la monnaie libre circule entre les mondes numériques et physiques.

---

## 🏗️ Architecture du Métaverse Mahala

```
┌─────────────────────────────────────────────────────────┐
│                    Mahala Blockchain                     │
│  - Consensus RVS                                         │
│  - Distribution DU                                       │
│  - Smart Contracts NFT                                   │
│  - Validation transactions                               │
└──────────────────┬──────────────────────────────────────┘
                   │
        ┌──────────┴──────────┐
        │                     │
┌───────▼────────┐   ┌────────▼────────┐   ┌──────────────┐
│  Serveurs      │   │   Bridge Mahala │   │   Dolibarr   │
│  Luanti        │◄──┤   ↔ Fiat        │◄──┤   ERP/CRM    │
│  - Mondes 3D   │   │   - KYC         │   │   - Boutiques│
│  - Téléport    │   │   - Achats      │   │   - Stocks   │
│  - Commerce    │   │   - Ventes       │   │   - Facturation│
│  - NFT Display │   │   - Frais 1%    │   │   - Livraison│
└────────────────┘   └─────────────────┘   └──────────────┘
```

---

## 🎮 Architecture Serveurs Luanti

### Grappe de serveurs interconnectés

```
              ┌─────────────────────┐
              │  Hub Central        │
              │  (spawn principal)  │
              │  Wallet: 0x...      │
              └──────────┬──────────┘
                         │
        ┌────────────────┼────────────────┐
        │                │                │
┌───────▼──────┐  ┌──────▼──────┐  ┌─────▼──────┐
│ Galerie      │  │ Monde RPG   │  │ Exposition │
│ Commerciale  │  │ Aventure    │  │ Art NFT    │
│              │  │             │  │            │
│ Coût entrée: │  │ Coût: 5 M   │  │ Gratuit    │
│ Gratuit      │  │             │  │            │
│              │  │             │  │            │
│ Location:    │  │ Items shop: │  │ Vente NFT: │
│ 50M/mois     │  │ 1-100 M     │  │ Variable   │
└──────────────┘  └─────────────┘  └────────────┘
```

### Configuration serveur Luanti

```ini
# minetest.conf pour serveur Mahala

# Connexion blockchain
mahala_node_url = https://node.mahala.org
mahala_wallet_address = MAHALA_SERVER_ADDRESS

# Téléportation
enable_teleport = true
teleport_base_cost = 1.0  # Mahala
teleport_distance_multiplier = 0.01

# Commerce
enable_marketplace = true
marketplace_fee = 0.02  # 2% des transactions

# NFT
enable_nft = true
nft_contract_address = 0x...

# Emplacements locatifs
enable_rental = true
rental_check_interval = 86400  # 1 jour
```

---

## 🎯 Système de Téléportation

### Architecture technique

```lua
-- mods/mahala_teleport/init.lua

local http = minetest.request_http_api()
local MAHALA_API = "https://api.mahala.org"

-- Calculer coût téléportation
function calculate_teleport_cost(from_world, to_world, distance)
    local base_cost = 1.0  -- 1 Mahala minimum
    local distance_cost = distance * 0.01
    local world_premium = get_world_premium(to_world)
    
    return base_cost + distance_cost + world_premium
end

-- Téléporter joueur
function teleport_player(player_name, from_world, to_world)
    local player = minetest.get_player_by_name(player_name)
    if not player then return false, "Player not found" end
    
    -- Récupérer wallet Mahala du joueur
    local wallet = get_player_wallet(player_name)
    if not wallet then 
        return false, "No Mahala wallet" 
    end
    
    -- Calculer coût
    local distance = calculate_distance(from_world, to_world)
    local cost = calculate_teleport_cost(from_world, to_world, distance)
    
    -- Vérifier balance
    local balance = check_mahala_balance(wallet)
    if balance < cost then
        return false, "Insufficient Mahala: need " .. cost .. " M"
    end
    
    -- Demander confirmation
    show_teleport_dialog(player, to_world, cost, function(confirmed)
        if not confirmed then return end
        
        -- Effectuer transaction blockchain
        local tx_result = http.fetch({
            url = MAHALA_API .. "/transaction/send",
            method = "POST",
            data = minetest.write_json({
                from = wallet,
                to = get_world_wallet(to_world),
                amount = cost,
                metadata = {
                    type = "teleport",
                    from_world = from_world,
                    to_world = to_world,
                    player = player_name
                }
            })
        })
        
        if tx_result.succeeded then
            -- Transaction validée, téléporter
            local target_server = get_server_address(to_world)
            minetest.send_join_request(player, target_server)
            
            minetest.chat_send_player(
                player_name,
                "Téléportation vers " .. to_world .. " (-" .. cost .. " M)"
            )
        else
            minetest.chat_send_player(
                player_name,
                "Erreur transaction: " .. tx_result.error
            )
        end
    end)
end

-- Interface téléportation
minetest.register_chatcommand("teleport", {
    params = "<world_name>",
    description = "Téléporter vers un autre monde (coût en Mahala)",
    func = function(name, param)
        if param == "" then
            -- Afficher liste des mondes disponibles
            show_world_list(name)
            return true
        end
        
        local current_world = minetest.get_current_world()
        teleport_player(name, current_world, param)
        return true
    end
})

-- GUI téléportation
function show_world_list(player_name)
    local worlds = get_available_worlds()
    local formspec = "size[10,8]" ..
                     "label[0.5,0.5;🌍 Mondes disponibles]" ..
                     "tableoptions[highlight=#00000000;background=#111111]" ..
                     "tablecolumns[text;text;text;text]" ..
                     "table[0.5,1;9,6;worlds_table;"
    
    -- Header
    formspec = formspec .. "Monde,Type,Population,Coût (M)"
    
    for _, world in ipairs(worlds) do
        local cost = calculate_teleport_cost(
            get_current_world(), 
            world.name, 
            world.distance
        )
        
        formspec = formspec .. "," .. 
                   world.name .. "," ..
                   world.type .. "," ..
                   world.player_count .. "," ..
                   string.format("%.2f", cost)
    end
    
    formspec = formspec .. ";]" ..
               "button[3.5,7.5;3,0.5;teleport;Téléporter]"
    
    minetest.show_formspec(player_name, "mahala:world_list", formspec)
end
```

---

## 🏪 Galeries Marchandes Virtuelles

### Système de location d'emplacements

```lua
-- mods/mahala_rental/init.lua

-- Structure emplacement
local Rental = {
    id = "",              -- ID unique
    world = "",           -- Monde
    position = {x, y, z}, -- Position 3D
    size = {x, y, z},     -- Dimensions
    owner = "",           -- Wallet propriétaire
    tenant = "",          -- Wallet locataire (nil si libre)
    price_per_day = 0,    -- Prix journalier en Mahala
    contract_end = 0,     -- Timestamp fin contrat
    metadata = {}         -- Infos supplémentaires
}

-- Créer zone locative
function create_rental_zone(world, pos1, pos2, price_per_day, metadata)
    local rental = {
        id = generate_rental_id(),
        world = world,
        position = pos1,
        size = calculate_size(pos1, pos2),
        owner = get_world_wallet(world),
        tenant = nil,
        price_per_day = price_per_day,
        contract_end = 0,
        metadata = metadata or {}
    }
    
    -- Sauvegarder sur blockchain (NFT)
    local nft_id = create_rental_nft(rental)
    rental.nft_id = nft_id
    
    -- Protéger zone
    protect_area(pos1, pos2, rental.id)
    
    -- Placer panneau "À louer"
    place_rental_sign(pos1, rental)
    
    save_rental(rental)
    return rental
end

-- Louer emplacement
function rent_space(player_name, rental_id, duration_days)
    local player = minetest.get_player_by_name(player_name)
    local rental = get_rental(rental_id)
    
    if not rental then
        return false, "Rental not found"
    end
    
    if rental.tenant then
        return false, "Already rented"
    end
    
    local wallet = get_player_wallet(player_name)
    local total_cost = rental.price_per_day * duration_days
    
    -- Vérifier balance
    local balance = check_mahala_balance(wallet)
    if balance < total_cost then
        return false, "Insufficient Mahala"
    end
    
    -- Transaction blockchain
    local tx = send_mahala_transaction(
        wallet,
        rental.owner,
        total_cost,
        {
            type = "rental_payment",
            rental_id = rental_id,
            duration_days = duration_days
        }
    )
    
    if tx.success then
        -- Mettre à jour contrat
        rental.tenant = wallet
        rental.contract_end = os.time() + (duration_days * 86400)
        save_rental(rental)
        
        -- Donner droits construction
        grant_build_rights(player_name, rental.id)
        
        -- Notification
        minetest.chat_send_player(
            player_name,
            "✅ Emplacement loué pour " .. duration_days .. 
            " jours (-" .. total_cost .. " M)"
        )
        
        return true, rental
    else
        return false, "Transaction failed"
    end
end

-- Vérification quotidienne des contrats
minetest.register_globalstep(function(dtime)
    -- Toutes les heures
    if os.time() % 3600 == 0 then
        check_rental_expirations()
    end
end)

function check_rental_expirations()
    local rentals = get_all_rentals()
    local now = os.time()
    
    for _, rental in ipairs(rentals) do
        if rental.tenant and rental.contract_end < now then
            -- Contrat expiré
            expire_rental(rental)
        end
    end
end

function expire_rental(rental)
    minetest.log("info", "Rental expired: " .. rental.id)
    
    -- Notifier locataire
    if rental.tenant then
        send_notification(
            rental.tenant,
            "Votre location a expiré: " .. rental.id
        )
    end
    
    -- Retirer droits
    revoke_build_rights(rental.tenant, rental.id)
    
    -- Nettoyer zone (optionnel)
    if rental.metadata.auto_clear then
        clear_rental_area(rental)
    end
    
    -- Libérer
    rental.tenant = nil
    rental.contract_end = 0
    save_rental(rental)
    
    -- Replacer panneau
    place_rental_sign(rental.position, rental)
end

-- Interface location
function show_rental_interface(player_name, rental_id)
    local rental = get_rental(rental_id)
    
    local formspec = "size[8,6]" ..
        "label[0.5,0.5;📦 Location d'emplacement]" ..
        "label[0.5,1;Monde: " .. rental.world .. "]" ..
        "label[0.5,1.5;Taille: " .. 
            rental.size.x .. "x" .. rental.size.y .. "x" .. rental.size.z .. "]" ..
        "label[0.5,2;Prix: " .. rental.price_per_day .. " M/jour]" ..
        "label[0.5,2.5;Statut: " .. (rental.tenant and "Occupé" or "Libre") .. "]"
    
    if not rental.tenant then
        formspec = formspec ..
            "field[0.8,3.5;3,0.5;duration;Durée (jours):;30]" ..
            "button[4,3.2;3,0.8;rent;Louer]"
    else
        local days_left = math.floor((rental.contract_end - os.time()) / 86400)
        formspec = formspec ..
            "label[0.5,3;Jours restants: " .. days_left .. "]"
        
        if get_player_wallet(player_name) == rental.tenant then
            formspec = formspec ..
                "button[2,4;4,0.8;extend;Prolonger]"
        end
    end
    
    minetest.show_formspec(player_name, "mahala:rental_" .. rental_id, formspec)
end
```

---

## 🎨 NFT Marketplace Intégré

### Smart Contract NFT sur Blockchain Mahala

```rust
// blockchain/src/nft/contract.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFT {
    pub id: String,
    pub owner: WalletAddress,
    pub creator: WalletAddress,
    pub metadata: NFTMetadata,
    pub created_at: u64,
    pub royalty_percentage: u8, // % pour créateur à chaque revente
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFTMetadata {
    pub name: String,
    pub description: String,
    pub nft_type: NFTType,
    pub media_url: String,      // IPFS hash ou URL
    pub thumbnail_url: String,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NFTType {
    Art,              // Art numérique
    Music,            // Musique
    Video,            // Vidéo
    Model3D,          // Modèle 3D pour Luanti
    VirtualLand,      // Terrain virtuel
    RentalContract,   // Contrat de location
    ServiceTicket,    // Bon pour service réel
    ProductVoucher,   // Bon d'achat produit
}

pub struct NFTContract {
    nfts: HashMap<String, NFT>,
    listings: HashMap<String, NFTListing>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFTListing {
    pub nft_id: String,
    pub seller: WalletAddress,
    pub price: f64,
    pub listed_at: u64,
    pub expires_at: Option<u64>,
}

impl NFTContract {
    pub fn new() -> Self {
        Self {
            nfts: HashMap::new(),
            listings: HashMap::new(),
        }
    }
    
    /// Minter un NFT
    pub fn mint(
        &mut self,
        creator: WalletAddress,
        metadata: NFTMetadata,
        royalty_percentage: u8,
    ) -> Result<String, NFTError> {
        if royalty_percentage > 10 {
            return Err(NFTError::RoyaltyTooHigh);
        }
        
        let nft_id = generate_nft_id();
        
        let nft = NFT {
            id: nft_id.clone(),
            owner: creator.clone(),
            creator,
            metadata,
            created_at: current_timestamp(),
            royalty_percentage,
        };
        
        self.nfts.insert(nft_id.clone(), nft);
        
        Ok(nft_id)
    }
    
    /// Lister un NFT à la vente
    pub fn list_for_sale(
        &mut self,
        nft_id: &str,
        seller: &WalletAddress,
        price: f64,
        duration_days: Option<u64>,
    ) -> Result<(), NFTError> {
        let nft = self.nfts.get(nft_id)
            .ok_or(NFTError::NotFound)?;
        
        if &nft.owner != seller {
            return Err(NFTError::NotOwner);
        }
        
        if self.listings.contains_key(nft_id) {
            return Err(NFTError::AlreadyListed);
        }
        
        let expires_at = duration_days.map(|days| {
            current_timestamp() + (days * 86400)
        });
        
        let listing = NFTListing {
            nft_id: nft_id.to_string(),
            seller: seller.clone(),
            price,
            listed_at: current_timestamp(),
            expires_at,
        };
        
        self.listings.insert(nft_id.to_string(), listing);
        
        Ok(())
    }
    
    /// Acheter un NFT
    pub fn buy_nft(
        &mut self,
        nft_id: &str,
        buyer: &WalletAddress,
        payment: f64,
    ) -> Result<NFTTransferResult, NFTError> {
        let listing = self.listings.get(nft_id)
            .ok_or(NFTError::NotListed)?;
        
        // Vérifier expiration
        if let Some(expires_at) = listing.expires_at {
            if current_timestamp() > expires_at {
                self.listings.remove(nft_id);
                return Err(NFTError::ListingExpired);
            }
        }
        
        // Vérifier prix
        if payment < listing.price {
            return Err(NFTError::InsufficientPayment);
        }
        
        let nft = self.nfts.get_mut(nft_id)
            .ok_or(NFTError::NotFound)?;
        
        // Calculer royalties
        let royalty_amount = (payment * nft.royalty_percentage as f64) / 100.0;
        let seller_amount = payment - royalty_amount;
        
        // Transférer NFT
        nft.owner = buyer.clone();
        
        // Retirer listing
        self.listings.remove(nft_id);
        
        Ok(NFTTransferResult {
            nft_id: nft_id.to_string(),
            new_owner: buyer.clone(),
            seller: listing.seller.clone(),
            creator: nft.creator.clone(),
            total_paid: payment,
            seller_receives: seller_amount,
            creator_receives: royalty_amount,
        })
    }
    
    /// Transférer NFT (gratuit)
    pub fn transfer(
        &mut self,
        nft_id: &str,
        from: &WalletAddress,
        to: &WalletAddress,
    ) -> Result<(), NFTError> {
        let nft = self.nfts.get_mut(nft_id)
            .ok_or(NFTError::NotFound)?;
        
        if &nft.owner != from {
            return Err(NFTError::NotOwner);
        }
        
        // Retirer listing si existe
        self.listings.remove(nft_id);
        
        nft.owner = to.clone();
        
        Ok(())
    }
    
    /// Obtenir NFTs d'un wallet
    pub fn get_nfts_by_owner(&self, owner: &WalletAddress) -> Vec<&NFT> {
        self.nfts.values()
            .filter(|nft| &nft.owner == owner)
            .collect()
    }
    
    /// Obtenir listings actifs
    pub fn get_active_listings(&self) -> Vec<(&NFT, &NFTListing)> {
        let now = current_timestamp();
        
        self.listings.values()
            .filter(|listing| {
                listing.expires_at.map_or(true, |exp| exp > now)
            })
            .filter_map(|listing| {
                self.nfts.get(&listing.nft_id)
                    .map(|nft| (nft, listing))
            })
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFTTransferResult {
    pub nft_id: String,
    pub new_owner: WalletAddress,
    pub seller: WalletAddress,
    pub creator: WalletAddress,
    pub total_paid: f64,
    pub seller_receives: f64,
    pub creator_receives: f64,
}

#[derive(Debug, thiserror::Error)]
pub enum NFTError {
    #[error("NFT not found")]
    NotFound,
    
    #[error("Not the owner")]
    NotOwner,
    
    #[error("Royalty percentage too high (max 10%)")]
    RoyaltyTooHigh,
    
    #[error("NFT already listed")]
    AlreadyListed,
    
    #[error("NFT not listed for sale")]
    NotListed,
    
    #[error("Listing expired")]
    ListingExpired,
    
    #[error("Insufficient payment")]
    InsufficientPayment,
}
```

### Affichage NFT dans Luanti

```lua
-- mods/mahala_nft/display.rs

-- Afficher NFT en 3D
function display_nft_in_world(pos, nft_id)
    local nft = fetch_nft_from_blockchain(nft_id)
    
    if not nft then
        return false
    end
    
    -- Créer cadre/piédestal
    local frame_node = "mahala_nft:frame"
    minetest.set_node(pos, {name = frame_node})
    
    -- Metadata du node
    local meta = minetest.get_meta(pos)
    meta:set_string("nft_id", nft_id)
    meta:set_string("nft_name", nft.metadata.name)
    meta:set_string("owner", nft.owner)
    meta:set_string("creator", nft.creator)
    
    -- Afficher image (si art 2D)
    if nft.metadata.nft_type == "Art" then
        display_texture_on_frame(pos, nft.metadata.media_url)
    end
    
    -- Afficher modèle 3D (si modèle 3D)
    if nft.metadata.nft_type == "Model3D" then
        spawn_3d_model(pos, nft.metadata.media_url)
    end
    
    -- Interaction
    set_nft_interaction(pos, nft)
    
    return true
end

-- Interaction avec NFT
function set_nft_interaction(pos, nft)
    minetest.register_node_interaction(pos, function(player)
        show_nft_info(player:get_player_name(), nft)
    end)
end

function show_nft_info(player_name, nft)
    local formspec = "size[10,8]" ..
        "label[0.5,0.5;" .. minetest.formspec_escape(nft.metadata.name) .. "]" ..
        "image[0.5,1;3,3;" .. nft.metadata.thumbnail_url .. "]" ..
        "label[4,1;Créateur: " .. short_address(nft.creator) .. "]" ..
        "label[4,1.5;Propriétaire: " .. short_address(nft.owner) .. "]" ..
        "label[4,2;Type: " .. nft.metadata.nft_type .. "]" ..
        "textarea[0.5,4.5;9,3;description;;" .. 
            minetest.formspec_escape(nft.metadata.description) .. "]"
    
    -- Si à vendre
    local listing = get_nft_listing(nft.id)
    if listing then
        formspec = formspec ..
            "label[4,2.5;Prix: " .. listing.price .. " M]" ..
            "button[3.5,7;3,0.8;buy;Acheter]"
    end
    
    -- Si propriétaire
    local player_wallet = get_player_wallet(player_name)
    if player_wallet == nft.owner then
        formspec = formspec ..
            "button[1,7;3,0.8;sell;Mettre en vente]" ..
            "button[6,7;3,0.8;transfer;Transférer]"
    end
    
    minetest.show_formspec(player_name, "mahala:nft_" .. nft.id, formspec)
end

-- Galerie NFT
function create_nft_gallery(world, pos1, pos2)
    local gallery = {
        world = world,
        area = {pos1 = pos1, pos2 = pos2},
        display_spots = {},
    }
    
    -- Créer spots d'affichage
    local spots = generate_display_spots(pos1, pos2, 3) -- Espacement 3 blocs
    
    for _, spot in ipairs(spots) do
        table.insert(gallery.display_spots, {
            position = spot,
            nft_id = nil,
            rented_by = nil,
            rental_price = 10.0, -- Mahala/jour
        })
    end
    
    save_gallery(gallery)
    return gallery
end
```

---

## 🛒 Passerelle Commerce Virtuel ↔ Réel

### Module Dolibarr Mahala Commerce

```php
// dolibarr-module/mahala/class/mahala_shop.class.php

class MahalaShop extends CommonObject {
    
    public $mahala_wallet;
    public $luanti_world;
    public $luanti_position;
    public $virtual_shop_id;
    public $fk_soc; // Société Dolibarr
    
    /**
     * Créer boutique virtuelle liée à Dolibarr
     */
    public function createVirtualShop($world, $rental_id) {
        global $conf;
        
        // Vérifier que société a wallet Mahala
        if (!$this->mahala_wallet) {
            return -1;
        }
        
        // Créer boutique dans Luanti via API
        $luanti_api = new LuantiAPI($conf->global->LUANTI_API_URL);
        
        $shop = $luanti_api->createShop([
            'world' => $world,
            'rental_id' => $rental_id,
            'owner_wallet' => $this->mahala_wallet,
            'shop_name' => $this->name,
            'logo_url' => $this->logo,
        ]);
        
        if ($shop) {
            $this->virtual_shop_id = $shop['id'];
            $this->luanti_world = $world;
            $this->luanti_position = $shop['position'];
            $this->update($user);
            
            return 1;
        }
        
        return -1;
    }
    
    /**
     * Synchroniser produits Dolibarr → Luanti
     */
    public function syncProductsToVirtual() {
        global $db;
        
        require_once DOL_DOCUMENT_ROOT.'/product/class/product.class.php';
        
        // Récupérer produits de la société
        $sql = "SELECT rowid FROM ".MAIN_DB_PREFIX."product";
        $sql .= " WHERE fk_soc = ".$this->fk_soc;
        $sql .= " AND tosell = 1"; // Seulement produits en vente
        
        $resql = $db->query($sql);
        
        if ($resql) {
            $products = [];
            
            while ($obj = $db->fetch_object($resql)) {
                $product = new Product($db);
                $product->fetch($obj->rowid);
                
                // Convertir en format Luanti
                $virtual_product = [
                    'id' => $product->id,
                    'name' => $product->label,
                    'description' => $product->description,
                    'price_mahala' => $this->convertToMahala($product->price),
                    'stock' => $product->stock_reel,
                    'image_url' => $this->getProductImageUrl($product),
                    'type' => $product->type == 0 ? 'physical' : 'service',
                ];
                
                $products[] = $virtual_product;
            }
            
            // Envoyer à Luanti
            $luanti_api = new LuantiAPI();
            $luanti_api->updateShopProducts($this->virtual_shop_id, $products);
            
            return count($products);
        }
        
        return -1;
    }
    
    /**
     * Traiter commande virtuelle
     */
    public function processVirtualOrder($order_data) {
        global $db, $user;
        
        require_once DOL_DOCUMENT_ROOT.'/commande/class/commande.class.php';
        
        // Créer commande Dolibarr
        $order = new Commande($db);
        $order->socid = $this->fk_soc;
        $order->date = dol_now();
        
        // Source commande
        $order->note_private = "Commande depuis monde virtuel: " . $this->luanti_world;
        $order->note_private .= "\nWallet client: " . $order_data['customer_wallet'];
        $order->note_private .= "\nTransaction Mahala: " . $order_data['tx_hash'];
        
        $order->create($user);
        
        // Ajouter lignes
        foreach ($order_data['items'] as $item) {
            $order->addline(
                $item['description'],
                $this->convertFromMahala($item['price_mahala']),
                $item['quantity'],
                0, // TVA
                0, // Local tax 1
                0, // Local tax 2
                $item['product_id']
            );
        }
        
        // Valider commande
        $order->valid($user);
        
        // Créer facture automatique
        $this->createInvoiceFromOrder($order, $order_data);
        
        // Notifier dans Luanti
        $this->notifyVirtualShop($order->id, 'order_received');
        
        return $order->id;
    }
    
    /**
     * Créer facture et enregistrer paiement Mahala
     */
    private function createInvoiceFromOrder($order, $order_data) {
        global $db, $user;
        
        require_once DOL_DOCUMENT_ROOT.'/compta/facture/class/facture.class.php';
        
        // Créer facture
        $invoice = new Facture($db);
        $invoice->socid = $order->socid;
        $invoice->date = dol_now();
        $invoice->cond_reglement_id = 1; // Paiement immédiat
        $invoice->mode_reglement_id = 1000; // ID mode "Mahala"
        
        $invoice->create($user);
        
        // Lier à commande
        $invoice->add_object_linked('order', $order->id);
        
        // Ajouter lignes depuis commande
        $order->fetch_lines();
        foreach ($order->lines as $line) {
            $invoice->addline(
                $line->description,
                $line->subprice,
                $line->qty,
                $line->tva_tx,
                $line->localtax1_tx,
                $line->localtax2_tx,
                $line->fk_product
            );
        }
        
        // Valider facture
        $invoice->validate($user);
        
        // Enregistrer paiement Mahala
        require_once DOL_DOCUMENT_ROOT.'/compta/paiement/class/paiement.class.php';
        
        $payment = new Paiement($db);
        $payment->datepaye = dol_now();
        $payment->amounts[$invoice->id] = $invoice->total_ttc;
        $payment->paiementid = 1000; // Type "Mahala"
        $payment->num_paiement = $order_data['tx_hash'];
        
        $payment->create($user);
        $payment->addPaymentToBank($user, 'payment', '', '', '', '');
        
        return $invoice->id;
    }
    
    /**
     * Convertir Euro → Mahala
     */
    private function convertToMahala($euro_amount) {
        // Récupérer taux depuis bridge
        $bridge_api = new MahalaBridgeAPI();
        $rate = $bridge_api->getEuroToMahalaRate();
        
        return $euro_amount * $rate;
    }
    
    /**
     * Convertir Mahala → Euro
     */
    private function convertFromMahala($mahala_amount) {
        $bridge_api = new MahalaBridgeAPI();
        $rate = $bridge_api->getMahalaToEuroRate();
        
        return $mahala_amount * $rate;
    }
}
```

### Interface Boutique dans Luanti

```lua
-- mods/mahala_shop/init.lua

-- Créer boutique virtuelle
function create_virtual_shop(pos, shop_data)
    -- Construire structure boutique
    build_shop_structure(pos, shop_data.size or {x=10, y=5, z=10})
    
    -- Placer panneau
    place_shop_sign(pos, shop_data.shop_name)
    
    -- Créer NPC vendeur
    spawn_shop_npc(pos, shop_data)
    
    -- Afficher produits
    display_products(pos, shop_data.products)
    
    -- Sauvegarder
    local shop = {
        id = shop_data.id,
        position = pos,
        owner_wallet = shop_data.owner_wallet,
        name = shop_data.shop_name,
        products = shop_data.products,
        dolibarr_id = shop_data.dolibarr_id,
    }
    
    save_shop(shop)
    return shop
end

-- Afficher catalogue produits
function display_products(shop_pos, products)
    local display_pos = vector.add(shop_pos, {x=2, y=1, z=2})
    
    for i, product in ipairs(products) do
        local pos = vector.add(display_pos, {
            x = (i % 3) * 3,
            y = 0,
            z = math.floor(i / 3) * 3
        })
        
        -- Créer présentoir
        minetest.set_node(pos, {name = "mahala_shop:display_stand"})
        
        local meta = minetest.get_meta(pos)
        meta:set_string("product_id", product.id)
        meta:set_string("product_name", product.name)
        meta:set_float("price_mahala", product.price_mahala)
        meta:set_int("stock", product.stock)
        
        -- Afficher image produit
        if product.image_url then
            display_product_image(pos, product.image_url)
        end
        
        -- Afficher prix
        spawn_price_label(vector.add(pos, {x=0, y=1, z=0}), product.price_mahala)
    end
end

-- Acheter produit
function buy_product(player_name, shop_id, product_id, quantity)
    local player = minetest.get_player_by_name(player_name)
    local shop = get_shop(shop_id)
    local product = get_shop_product(shop_id, product_id)
    
    if not product then
        return false, "Produit introuvable"
    end
    
    if product.stock < quantity then
        return false, "Stock insuffisant"
    end
    
    local total_price = product.price_mahala * quantity
    local customer_wallet = get_player_wallet(player_name)
    
    -- Vérifier balance
    local balance = check_mahala_balance(customer_wallet)
    if balance < total_price then
        return false, "Mahala insuffisants"
    end
    
    -- Transaction blockchain
    local tx = send_mahala_transaction(
        customer_wallet,
        shop.owner_wallet,
        total_price,
        {
            type = "shop_purchase",
            shop_id = shop_id,
            product_id = product_id,
            quantity = quantity,
            product_type = product.type
        }
    )
    
    if tx.success then
        -- Déduire stock localement
        product.stock = product.stock - quantity
        update_product_stock(shop_id, product_id, product.stock)
        
        -- Notifier Dolibarr (si produit physique)
        if product.type == "physical" then
            notify_dolibarr_order({
                shop_id = shop_id,
                customer_wallet = customer_wallet,
                customer_name = player_name,
                items = {{
                    product_id = product_id,
                    quantity = quantity,
                    price_mahala = product.price_mahala
                }},
                tx_hash = tx.hash,
                total_mahala = total_price,
                delivery_world = minetest.get_current_world(),
                delivery_player = player_name
            })
            
            minetest.chat_send_player(
                player_name,
                "✅ Commande validée ! Livraison IRL dans 2-5 jours"
            )
        else
            -- Service numérique immédiat
            deliver_digital_product(player_name, product)
            
            minetest.chat_send_player(
                player_name,
                "✅ Achat réussi ! Service activé"
            )
        end
        
        return true
    else
        return false, "Transaction échouée"
    end
end

-- Interface boutique
function show_shop_catalog(player_name, shop_id)
    local shop = get_shop(shop_id)
    
    local formspec = "size[12,10]" ..
        "label[0.5,0.5;🏪 " .. shop.name .. "]" ..
        "label[0.5,1;Propriétaire: " .. short_address(shop.owner_wallet) .. "]"
    
    -- Tableau produits
    formspec = formspec ..
        "tablecolumns[image,width=1;text;text;text;text]" ..
        "table[0.5,1.5;11,7;products;"
    
    local first = true
    for _, product in ipairs(shop.products) do
        if not first then formspec = formspec .. "," end
        
        formspec = formspec ..
            product.image_url .. "," ..
            product.name .. "," ..
            product.price_mahala .. " M," ..
            "Stock: " .. product.stock .. "," ..
            product.type
        
        first = false
    end
    
    formspec = formspec .. ";]" ..
        "button[4,9;4,0.8;buy;Acheter]"
    
    minetest.show_formspec(player_name, "mahala:shop_" .. shop_id, formspec)
end
```

---

## 💳 Service d'Échange Mahala ↔ Fiat

### Bridge Mahala ↔ Euro avec KYC

```rust
// bridge/src/fiat_gateway.rs

use serde::{Deserialize, Serialize};

pub struct FiatGateway {
    kyc_provider: KYCProvider,
    payment_processor: PaymentProcessor,
    fee_percentage: f64, // 1%
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KYCVerification {
    pub user_id: String,
    pub wallet_address: String,
    pub status: KYCStatus,
    pub verified_at: Option<u64>,
    pub documents: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KYCStatus {
    Pending,
    Verified,
    Rejected,
}

impl FiatGateway {
    pub fn new() -> Self {
        Self {
            kyc_provider: KYCProvider::new(),
            payment_processor: PaymentProcessor::new(),
            fee_percentage: 0.01, // 1%
        }
    }
    
    /// Acheter Mahala avec carte bancaire
    pub async fn buy_mahala_with_card(
        &self,
        user: &User,
        euro_amount: f64,
    ) -> Result<PurchaseResult, GatewayError> {
        // Vérifier KYC
        self.verify_kyc_status(&user.id).await?;
        
        // Limites anti-blanchiment
        self.check_limits(user, euro_amount)?;
        
        // Traiter paiement
        let payment = self.payment_processor
            .charge_card(&user.card_token, euro_amount)
            .await?;
        
        if !payment.success {
            return Err(GatewayError::PaymentFailed);
        }
        
        // Calculer Mahala (moins frais)
        let fee = euro_amount * self.fee_percentage;
        let net_amount = euro_amount - fee;
        let mahala_amount = self.calculate_mahala_amount(net_amount).await?;
        
        // Envoyer Mahala
        let tx = self.send_mahala_to_user(&user.wallet, mahala_amount).await?;
        
        // Enregistrer transaction
        self.record_fiat_transaction(FiatTransaction {
            user_id: user.id.clone(),
            direction: TransactionDirection::FiatToMahala,
            fiat_amount: euro_amount,
            fiat_currency: "EUR".to_string(),
            mahala_amount,
            fee,
            tx_hash: tx.hash,
            payment_method: "card".to_string(),
            timestamp: current_timestamp(),
        }).await?;
        
        Ok(PurchaseResult {
            mahala_received: mahala_amount,
            euro_charged: euro_amount,
            fee,
            tx_hash: tx.hash,
        })
    }
    
    /// Vendre Mahala pour EUR
    pub async fn sell_mahala_for_euro(
        &self,
        user: &User,
        mahala_amount: f64,
    ) -> Result<SaleResult, GatewayError> {
        // Vérifier KYC
        self.verify_kyc_status(&user.id).await?;
        
        // Vérifier balance Mahala
        let balance = check_mahala_balance(&user.wallet)?;
        if balance < mahala_amount {
            return Err(GatewayError::InsufficientBalance);
        }
        
        // Calculer EUR (moins frais)
        let euro_value = self.calculate_euro_value(mahala_amount).await?;
        let fee = euro_value * self.fee_percentage;
        let net_euro = euro_value - fee;
        
        // Recevoir Mahala de l'utilisateur
        let tx = self.receive_mahala_from_user(&user.wallet, mahala_amount).await?;
        
        // Envoyer EUR (virement SEPA)
        let payout = self.payment_processor
            .send_sepa_transfer(&user.iban, net_euro)
            .await?;
        
        // Enregistrer
        self.record_fiat_transaction(FiatTransaction {
            user_id: user.id.clone(),
            direction: TransactionDirection::MahalaToFiat,
            fiat_amount: euro_value,
            fiat_currency: "EUR".to_string(),
            mahala_amount,
            fee,
            tx_hash: tx.hash,
            payment_method: "sepa".to_string(),
            timestamp: current_timestamp(),
        }).await?;
        
        Ok(SaleResult {
            mahala_sold: mahala_amount,
            euro_received: net_euro,
            fee,
            payout_reference: payout.reference,
        })
    }
    
    /// Vérifier statut KYC
    async fn verify_kyc_status(&self, user_id: &str) -> Result<(), GatewayError> {
        let kyc = self.kyc_provider.get_verification(user_id).await?;
        
        match kyc.status {
            KYCStatus::Verified => Ok(()),
            KYCStatus::Pending => Err(GatewayError::KYCPending),
            KYCStatus::Rejected => Err(GatewayError::KYCRejected),
        }
    }
    
    /// Limites anti-blanchiment
    fn check_limits(&self, user: &User, amount: f64) -> Result<(), GatewayError> {
        // Limite quotidienne: 1000 EUR
        if amount > 1000.0 {
            return Err(GatewayError::DailyLimitExceeded);
        }
        
        // Limite mensuelle: 5000 EUR
        let monthly = self.get_monthly_volume(&user.id)?;
        if monthly + amount > 5000.0 {
            return Err(GatewayError::MonthlyLimitExceeded);
        }
        
        Ok(())
    }
    
    async fn calculate_mahala_amount(&self, euro: f64) -> Result<f64, GatewayError> {
        // Récupérer taux du bridge
        let rate = self.get_exchange_rate().await?;
        Ok(euro * rate)
    }
    
    async fn calculate_euro_value(&self, mahala: f64) -> Result<f64, GatewayError> {
        let rate = self.get_exchange_rate().await?;
        Ok(mahala / rate)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FiatTransaction {
    pub user_id: String,
    pub direction: TransactionDirection,
    pub fiat_amount: f64,
    pub fiat_currency: String,
    pub mahala_amount: f64,
    pub fee: f64,
    pub tx_hash: String,
    pub payment_method: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionDirection {
    FiatToMahala,
    MahalaToFiat,
}
```

---

## 📊 Tableau de Bord Dolibarr Métaverse

```php
// dolibarr-module/mahala/dashboard.php

require 'main.inc.php';
require_once DOL_DOCUMENT_ROOT.'/custom/mahala/class/mahala_stats.class.php';

$stats = new MahalaStats($db);

// Récupérer statistiques
$virtual_shops = $stats->getVirtualShopsCount();
$total_sales_mahala = $stats->getTotalSalesMahala();
$rental_income = $stats->getRentalIncome();
$nft_sales = $stats->getNFTSales();

llxHeader('', 'Dashboard Mahala Métaverse');

?>

<div class="fiche">
    <h1>🌐 Dashboard Mahala Métaverse</h1>
    
    <!-- KPIs -->
    <div class="fichecenter">
        <table class="border centpercent">
            <tr class="liste_titre">
                <th colspan="4">Indicateurs clés</th>
            </tr>
            <tr>
                <td width="25%">
                    <div class="box-flex-item">
                        <div class="box-flex-item-title">Boutiques Virtuelles</div>
                        <div class="box-flex-item-value"><?php echo $virtual_shops; ?></div>
                    </div>
                </td>
                <td width="25%">
                    <div class="box-flex-item">
                        <div class="box-flex-item-title">Ventes (Mahala)</div>
                        <div class="box-flex-item-value"><?php echo price($total_sales_mahala); ?> M</div>
                    </div>
                </td>
                <td width="25%">
                    <div class="box-flex-item">
                        <div class="box-flex-item-title">Revenus Locations</div>
                        <div class="box-flex-item-value"><?php echo price($rental_income); ?> M</div>
                    </div>
                </td>
                <td width="25%">
                    <div class="box-flex-item">
                        <div class="box-flex-item-title">Ventes NFT</div>
                        <div class="box-flex-item-value"><?php echo price($nft_sales); ?> M</div>
                    </div>
                </td>
            </tr>
        </table>
    </div>
    
    <!-- Liste boutiques virtuelles -->
    <div class="fichecenter">
        <h2>Mes Boutiques Virtuelles</h2>
        <?php
        $shops = $stats->getMyVirtualShops($user->societe_id);
        
        if (count($shops) > 0) {
            ?>
            <table class="border centpercent">
                <tr class="liste_titre">
                    <th>Monde</th>
                    <th>Position</th>
                    <th>Ventes (7j)</th>
                    <th>Visiteurs (7j)</th>
                    <th>Actions</th>
                </tr>
                <?php
                foreach ($shops as $shop) {
                    ?>
                    <tr>
                        <td><?php echo $shop->luanti_world; ?></td>
                        <td><?php echo $shop->luanti_position; ?></td>
                        <td><?php echo price($shop->sales_7d); ?> M</td>
                        <td><?php echo $shop->visitors_7d; ?></td>
                        <td>
                            <a href="shop.php?id=<?php echo $shop->id; ?>">Gérer</a>
                            <a href="<?php echo $shop->luanti_url; ?>" target="_blank">Visiter</a>
                        </td>
                    </tr>
                    <?php
                }
                ?>
            </table>
            <?php
        } else {
            ?>
            <div class="info">
                Vous n'avez pas encore de boutique virtuelle.
                <a href="shop_create.php">Créer ma première boutique</a>
            </div>
            <?php
        }
        ?>
    </div>
    
    <!-- Mes NFT -->
    <div class="fichecenter">
        <h2>Mes NFT</h2>
        <?php
        $nfts = $stats->getMyNFTs($user->entity);
        
        if (count($nfts) > 0) {
            ?>
            <div class="nft-grid">
                <?php foreach ($nfts as $nft) { ?>
                    <div class="nft-card">
                        <img src="<?php echo $nft->thumbnail_url; ?>" alt="<?php echo $nft->name; ?>">
                        <h3><?php echo $nft->name; ?></h3>
                        <p><?php echo $nft->type; ?></p>
                        <?php if ($nft->listed_price) { ?>
                            <div class="price">En vente: <?php echo $nft->listed_price; ?> M</div>
                        <?php } ?>
                        <a href="nft.php?id=<?php echo $nft->id; ?>">Détails</a>
                    </div>
                <?php } ?>
            </div>
            <?php
        }
        ?>
    </div>
</div>

<?php
llxFooter();
?>
```

---

## 🎯 Cas d'Usage Complets

### 1. Restaurant avec vitrine virtuelle

**Monde réel** : Restaurant "Chez Marcel" à Paris  
**Monde virtuel** : Boutique dans "Galerie Gastronomie"  

**Flow** :
1. Marcel loue emplacement galerie : 50 M/mois
2. Configure boutique Dolibarr liée
3. Sync menu avec photos sur présentoirs virtuels
4. Client visite monde Luanti, voit menu 3D
5. Commande plat : paiement 15 M
6. Transaction validée blockchain
7. Dolibarr crée commande + facture
8. Marcel prépare, client récupère IRL

### 2. Designer vendant modèles 3D

**Créateur** : Sophie, designer 3D  
**Produit** : Meubles pour Luanti  

**Flow** :
1. Sophie crée chaise 3D dans Blender
2. Mint NFT sur blockchain Mahala : 2 M (frais mint)
3. Liste NFT marketplace : 50 M
4. Met modèle en exposition galerie art
5. Acheteur achète NFT : 50 M
6. Sophie reçoit 49 M (1 M frais marketplace)
7. Acheteur peut placer modèle dans son monde
8. Reventes futures : Sophie touche 5% royalties

### 3. Consultant vendant service via métaverse

**Consultant** : Thomas, expert Dolibarr  
**Service** : Audit configuration  

**Flow** :
1. Thomas crée "service ticket" NFT : Audit 2h
2. Prix : 200 M (~200€)
3. Client achète ticket dans boutique virtuelle
4. NFT transféré au client = droit au service
5. Client présente NFT à Thomas
6. Thomas "brûle" NFT après service rendu
7. Dolibarr génère facture automatique
8. Thomas déclare revenu en EUR

---

## 📈 Modèle Économique Aplose

### Revenus Service

1. **Bridge Mahala ↔ Fiat** : 1% sur achats/ventes
2. **Marketplace NFT** : 2% sur transactions
3. **Hébergement serveurs Luanti** : 20€/mois
4. **Module Dolibarr Premium** : 10€/mois (gestion boutiques)
5. **KYC/Vérification** : 5€ one-time

### Exemple projection (an 1)

- 1000 utilisateurs actifs
- 500 boutiques virtuelles (10€/mois) = 5k€/mois
- 100k€ volume fiat/mois (1% frais) = 1k€/mois
- 50k M transactions marketplace (2%) = variable
- **Total estimé : 6-10k€/mois ARR**

---

## 🚀 Roadmap Métaverse

### Phase 1 : Infrastructure (Mois 1-3)
- ✅ Blockchain Mahala fonctionnelle
- ✅ Smart contracts NFT
- ✅ Serveurs Luanti + mods de base

### Phase 2 : Commerce (Mois 4-6)
- ✅ Téléportation payante
- ✅ Système location emplacements
- ✅ Module Dolibarr boutiques virtuelles
- ✅ Marketplace NFT

### Phase 3 : Bridge Fiat (Mois 7-9)
- ✅ KYC provider intégré
- ✅ Achat/vente Mahala carte bancaire
- ✅ SEPA out pour retraits
- ✅ Conformité réglementaire

### Phase 4 : Scaling (Mois 10-12)
- ⬜ 10+ mondes Luanti
- ⬜ 100+ boutiques
- ⬜ Cross-world teleportation
- ⬜ Events communautaires

---

## 🔒 Conformité & Sécurité

### Réglementaire
- **PSAN** (France) : Enregistrement obligatoire
- **KYC/AML** : Vérification identité >1000€
- **TVA** : Sur ventes biens/services réels
- **RGPD** : Protection données utilisateurs

### Sécurité Transactions
- Multi-sig wallets serveurs
- Rate limiting achats
- Détection patterns suspects
- Audit trail complet

---

**Ce document étend le plan Mahala avec l'intégration complète du métaverse Luanti et du commerce réel via Dolibarr, créant un écosystème économique unique où la monnaie libre circule librement entre monde virtuel et physique.**
