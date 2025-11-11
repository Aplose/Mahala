-- Mod Mahala Téléportation
-- Permet de téléporter entre les mondes Luanti avec paiement en Mahala

local http = minetest.request_http_api()
if not http then
    error("Mahala Teleport requires HTTP API")
end

local MAHALA_API = minetest.settings:get("mahala_node_url") or "http://localhost:8080"
local TELEPORT_BASE_COST = tonumber(minetest.settings:get("teleport_base_cost")) or 1.0
local TELEPORT_DISTANCE_MULTIPLIER = tonumber(minetest.settings:get("teleport_distance_multiplier")) or 0.01

-- Stockage des wallets des joueurs
local player_wallets = {}

-- Obtenir le wallet d'un joueur
local function get_player_wallet(player_name)
    return player_wallets[player_name]
end

-- Enregistrer le wallet d'un joueur
local function set_player_wallet(player_name, wallet_address)
    player_wallets[player_name] = wallet_address
    -- Sauvegarder dans le stockage du joueur
    local player_data = minetest.get_player_meta(player_name)
    player_data:set_string("mahala_wallet", wallet_address)
end

-- Charger le wallet au login
minetest.register_on_joinplayer(function(player)
    local player_name = player:get_player_name()
    local player_data = minetest.get_player_meta(player_name)
    local wallet = player_data:get_string("mahala_wallet")
    if wallet and wallet ~= "" then
        player_wallets[player_name] = wallet
    end
end)

-- Calculer le coût de téléportation
local function calculate_teleport_cost(from_world, to_world, distance)
    local base_cost = TELEPORT_BASE_COST
    local distance_cost = distance * TELEPORT_DISTANCE_MULTIPLIER
    local world_premium = 0.0 -- À implémenter selon le monde
    
    return base_cost + distance_cost + world_premium
end

-- Calculer la distance entre deux mondes
local function calculate_distance(from_world, to_world)
    -- Pour l'instant, distance fixe
    -- Dans une vraie implémentation, on utiliserait les coordonnées des serveurs
    return 100.0
end

-- Obtenir l'adresse du wallet d'un monde
local function get_world_wallet(world_name)
    -- Récupérer depuis la config ou la base de données
    return minetest.settings:get("mahala_wallet_address") or ""
end

-- Vérifier la balance Mahala
local function check_mahala_balance(wallet_address)
    local url = MAHALA_API .. "/blockchain/balance/" .. wallet_address
    
    local response = http.fetch({
        url = url,
        method = "GET",
        timeout = 5
    })
    
    if response and response.succeeded then
        local data = minetest.parse_json(response.data)
        if data and data.balance then
            return data.balance
        end
    end
    
    return 0.0
end

-- Envoyer une transaction Mahala
local function send_mahala_transaction(from, to, amount, metadata)
    local url = MAHALA_API .. "/transaction/submit"
    
    local tx_data = {
        transaction = {
            from = from,
            to = to,
            amount = amount,
            fee = 0.1,
            timestamp = os.time(),
            metadata = metadata or {}
        }
    }
    
    local response = http.fetch({
        url = url,
        method = "POST",
        data = minetest.write_json(tx_data),
        timeout = 10
    })
    
    if response and response.succeeded then
        local data = minetest.parse_json(response.data)
        return data and data.status == "accepted"
    end
    
    return false
end

-- Téléporter un joueur
local function teleport_player(player_name, from_world, to_world)
    local player = minetest.get_player_by_name(player_name)
    if not player then
        return false, "Joueur introuvable"
    end
    
    -- Récupérer le wallet
    local wallet = get_player_wallet(player_name)
    if not wallet or wallet == "" then
        return false, "Aucun wallet Mahala configuré. Utilisez /wallet <adresse>"
    end
    
    -- Calculer le coût
    local distance = calculate_distance(from_world, to_world)
    local cost = calculate_teleport_cost(from_world, to_world, distance)
    
    -- Vérifier la balance
    local balance = check_mahala_balance(wallet)
    if balance < cost then
        return false, "Mahala insuffisants: besoin de " .. string.format("%.2f", cost) .. " M"
    end
    
    -- Obtenir l'adresse du wallet du monde de destination
    local world_wallet = get_world_wallet(to_world)
    if not world_wallet or world_wallet == "" then
        return false, "Wallet du monde de destination non configuré"
    end
    
    -- Effectuer la transaction
    local tx_success = send_mahala_transaction(
        wallet,
        world_wallet,
        cost,
        {
            type = "teleport",
            from_world = from_world,
            to_world = to_world,
            player = player_name
        }
    )
    
    if not tx_success then
        return false, "Erreur lors de la transaction"
    end
    
    -- Téléporter (dans une vraie implémentation, on se connecterait à l'autre serveur)
    minetest.chat_send_player(
        player_name,
        "✅ Téléportation vers " .. to_world .. " (-" .. string.format("%.2f", cost) .. " M)"
    )
    
    return true, "Téléportation réussie"
end

-- Obtenir la liste des mondes disponibles
local function get_available_worlds()
    -- Pour l'instant, liste statique
    -- Dans une vraie implémentation, on récupérerait depuis une API
    return {
        {name = "Hub Central", type = "Hub", player_count = 10, distance = 0},
        {name = "Galerie Commerciale", type = "Commerce", player_count = 5, distance = 50},
        {name = "Monde RPG", type = "Aventure", player_count = 8, distance = 100},
        {name = "Exposition Art NFT", type = "Galerie", player_count = 3, distance = 75}
    }
end

-- Afficher la liste des mondes
local function show_world_list(player_name)
    local worlds = get_available_worlds()
    local current_world = minetest.settings:get("world_name") or "Monde Actuel"
    
    local formspec = "size[10,8]" ..
        "label[0.5,0.5;🌍 Mondes disponibles]" ..
        "tableoptions[highlight=#00000000;background=#111111]" ..
        "tablecolumns[text;text;text;text]" ..
        "table[0.5,1;9,6;worlds_table;"
    
    -- Header
    formspec = formspec .. "Monde,Type,Population,Coût (M)"
    
    for _, world in ipairs(worlds) do
        local cost = calculate_teleport_cost(current_world, world.name, world.distance)
        formspec = formspec .. "," ..
            world.name .. "," ..
            world.type .. "," ..
            tostring(world.player_count) .. "," ..
            string.format("%.2f", cost)
    end
    
    formspec = formspec .. ";]" ..
        "button[3.5,7.5;3,0.5;teleport;Téléporter]"
    
    minetest.show_formspec(player_name, "mahala:world_list", formspec)
end

-- Gérer les actions du formspec
minetest.register_on_player_receive_fields(function(player, formname, fields)
    if formname ~= "mahala:world_list" then
        return false
    end
    
    local player_name = player:get_player_name()
    
    if fields.teleport then
        local selected = fields.worlds_table
        if selected and selected ~= "" then
            local world_name = string.match(selected, "^([^,]+)")
            local current_world = minetest.settings:get("world_name") or "Monde Actuel"
            local success, msg = teleport_player(player_name, current_world, world_name)
            if not success then
                minetest.chat_send_player(player_name, "❌ " .. msg)
            end
        end
    end
    
    return true
end)

-- Commande de téléportation
minetest.register_chatcommand("teleport", {
    params = "<world_name>",
    description = "Téléporter vers un autre monde (coût en Mahala)",
    func = function(name, param)
        if param == "" then
            show_world_list(name)
            return true
        end
        
        local current_world = minetest.settings:get("world_name") or "Monde Actuel"
        local success, msg = teleport_player(name, current_world, param)
        
        if not success then
            return false, msg
        end
        
        return true, msg
    end
})

-- Commande pour configurer le wallet
minetest.register_chatcommand("wallet", {
    params = "<adresse>",
    description = "Configurer votre adresse wallet Mahala",
    func = function(name, param)
        if param == "" then
            local wallet = get_player_wallet(name)
            if wallet then
                return true, "Votre wallet: " .. wallet
            else
                return false, "Aucun wallet configuré. Utilisez /wallet <adresse>"
            end
        end
        
        -- Valider le format de l'adresse (hex 64 caractères)
        if string.len(param) ~= 64 or not string.match(param, "^[0-9a-fA-F]+$") then
            return false, "Adresse invalide. Format attendu: 64 caractères hexadécimaux"
        end
        
        set_player_wallet(name, param)
        return true, "Wallet configuré: " .. string.sub(param, 1, 20) .. "..."
    end
})

-- Commande pour vérifier la balance
minetest.register_chatcommand("balance", {
    description = "Vérifier votre balance Mahala",
    func = function(name, param)
        local wallet = get_player_wallet(name)
        if not wallet or wallet == "" then
            return false, "Aucun wallet configuré. Utilisez /wallet <adresse>"
        end
        
        local balance = check_mahala_balance(wallet)
        return true, "Balance: " .. string.format("%.2f", balance) .. " M"
    end
})

print("[Mahala Teleport] Mod chargé")

