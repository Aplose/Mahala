-- Mod Mahala Shop
-- Boutiques virtuelles liées à Dolibarr

local http = minetest.request_http_api()
if not http then
    error("Mahala Shop requires HTTP API")
end

local MAHALA_API = minetest.settings:get("mahala_node_url") or "http://localhost:8080"
local DOLIBARR_API = minetest.settings:get("dolibarr_api_url") or ""

-- Stockage des boutiques
local shops = {}
local shop_file = minetest.get_worldpath() .. "/mahala_shops.json"

-- Charger les boutiques
local function load_shops()
    local file = io.open(shop_file, "r")
    if file then
        local content = file:read("*all")
        file:close()
        if content and content ~= "" then
            shops = minetest.parse_json(content) or {}
        end
    end
end

-- Sauvegarder les boutiques
local function save_shops()
    local file = io.open(shop_file, "w")
    if file then
        file:write(minetest.write_json(shops))
        file:close()
    end
end

load_shops()

-- Créer une boutique virtuelle
local function create_virtual_shop(pos, shop_data)
    local shop = {
        id = minetest.pos_to_string(pos) .. "_" .. os.time(),
        position = pos,
        owner_wallet = shop_data.owner_wallet or "",
        name = shop_data.shop_name or "Boutique",
        products = shop_data.products or {},
        dolibarr_id = shop_data.dolibarr_id or ""
    }
    
    -- Construire la structure de la boutique
    for x = 0, 9 do
        for z = 0, 9 do
            local p = {x = pos.x + x, y = pos.y, z = pos.z + z}
            minetest.set_node(p, {name = "mahala_shop:floor"})
        end
    end
    
    -- Placer le panneau
    local sign_pos = {x = pos.x + 4, y = pos.y + 1, z = pos.z}
    minetest.set_node(sign_pos, {name = "mahala_shop:sign"})
    local meta = minetest.get_meta(sign_pos)
    meta:set_string("shop_id", shop.id)
    meta:set_string("infotext", shop.name)
    
    shops[shop.id] = shop
    save_shops()
    
    return shop
end

-- Obtenir une boutique
local function get_shop(shop_id)
    return shops[shop_id]
end

-- Acheter un produit
local function buy_product(player_name, shop_id, product_id, quantity)
    local shop = get_shop(shop_id)
    if not shop then
        return false, "Boutique introuvable"
    end
    
    local product = nil
    for _, p in ipairs(shop.products) do
        if p.id == product_id then
            product = p
            break
        end
    end
    
    if not product then
        return false, "Produit introuvable"
    end
    
    if product.stock < quantity then
        return false, "Stock insuffisant"
    end
    
    local total_price = product.price_mahala * quantity
    
    -- Récupérer le wallet du joueur
    local player_data = minetest.get_player_meta(player_name)
    local customer_wallet = player_data:get_string("mahala_wallet")
    if not customer_wallet or customer_wallet == "" then
        return false, "Aucun wallet configuré"
    end
    
    -- Vérifier la balance (simplifié)
    -- Transaction blockchain
    local tx_success = true -- Simplifié
    
    if tx_success then
        -- Déduire le stock
        product.stock = product.stock - quantity
        
        -- Notifier Dolibarr si produit physique
        if product.type == "physical" and DOLIBARR_API ~= "" then
            -- TODO: Appeler l'API Dolibarr pour créer la commande
        end
        
        save_shops()
        
        minetest.chat_send_player(
            player_name,
            "✅ Achat réussi: " .. product.name .. " x" .. quantity .. " (-" .. total_price .. " M)"
        )
        
        return true
    else
        return false, "Transaction échouée"
    end
end

-- Afficher le catalogue
local function show_shop_catalog(player_name, shop_id)
    local shop = get_shop(shop_id)
    if not shop then
        return
    end
    
    local formspec = "size[12,10]" ..
        "label[0.5,0.5;🏪 " .. shop.name .. "]" ..
        "label[0.5,1;Propriétaire: " .. string.sub(shop.owner_wallet, 1, 20) .. "...]"
    
    -- Tableau produits
    formspec = formspec ..
        "tablecolumns[text;text;text;text]" ..
        "table[0.5,1.5;11,7;products;"
    
    local first = true
    for _, product in ipairs(shop.products) do
        if not first then formspec = formspec .. "," end
        formspec = formspec ..
            product.name .. "," ..
            string.format("%.2f", product.price_mahala) .. " M," ..
            "Stock: " .. product.stock .. "," ..
            product.type
        first = false
    end
    
    formspec = formspec .. ";]" ..
        "field[0.5,8.5;3,0.5;product_id;ID Produit:;]" ..
        "field[4,8.5;3,0.5;quantity;Quantité:;1]" ..
        "button[8,8.2;3,0.8;buy;Acheter]"
    
    minetest.show_formspec(player_name, "mahala:shop_" .. shop_id, formspec)
end

-- Node pour le panneau de boutique
minetest.register_node("mahala_shop:sign", {
    description = "Panneau Boutique Mahala",
    tiles = {"mahala_shop_sign.png"},
    groups = {dig_immediate = 2},
    paramtype = "light",
    on_punch = function(pos, node, puncher, pointed_thing)
        local meta = minetest.get_meta(pos)
        local shop_id = meta:get_string("shop_id")
        if shop_id and shop_id ~= "" then
            show_shop_catalog(puncher:get_player_name(), shop_id)
        end
    end
})

-- Node pour le sol de la boutique
minetest.register_node("mahala_shop:floor", {
    description = "Sol Boutique",
    tiles = {"mahala_shop_floor.png"},
    groups = {dig_immediate = 2}
})

-- Gérer les actions
minetest.register_on_player_receive_fields(function(player, formname, fields)
    if string.find(formname, "mahala:shop_") then
        local shop_id = string.sub(formname, 12) -- Enlever "mahala:shop_"
        
        if fields.buy then
            local product_id = fields.product_id or ""
            local quantity = tonumber(fields.quantity) or 1
            
            if product_id ~= "" then
                local success, msg = buy_product(player:get_player_name(), shop_id, product_id, quantity)
                if not success then
                    minetest.chat_send_player(player:get_player_name(), "❌ " .. msg)
                end
            end
        end
        
        return true
    end
end)

-- Commande admin pour créer une boutique
minetest.register_chatcommand("create_shop", {
    params = "<name>",
    description = "Créer une boutique virtuelle (admin)",
    privs = {server = true},
    func = function(name, param)
        local shop_name = param or "Boutique"
        local player = minetest.get_player_by_name(name)
        if not player then
            return false, "Joueur introuvable"
        end
        
        local pos = player:get_pos()
        local shop_pos = {
            x = math.floor(pos.x - 5),
            y = math.floor(pos.y - 1),
            z = math.floor(pos.z - 5)
        }
        
        local shop_data = {
            shop_name = shop_name,
            owner_wallet = minetest.settings:get("mahala_wallet_address") or "",
            products = {}
        }
        
        local shop = create_virtual_shop(shop_pos, shop_data)
        return true, "Boutique créée: " .. shop.id
    end
})

print("[Mahala Shop] Mod chargé")

