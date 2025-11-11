-- Mod Mahala Location
-- Système de location d'emplacements dans les mondes Luanti

local http = minetest.request_http_api()
if not http then
    error("Mahala Rental requires HTTP API")
end

local MAHALA_API = minetest.settings:get("mahala_node_url") or "http://localhost:8080"

-- Stockage des locations
local rentals = {}
local rental_file = minetest.get_worldpath() .. "/mahala_rentals.json"

-- Charger les locations
local function load_rentals()
    local file = io.open(rental_file, "r")
    if file then
        local content = file:read("*all")
        file:close()
        if content and content ~= "" then
            rentals = minetest.parse_json(content) or {}
        end
    end
end

-- Sauvegarder les locations
local function save_rentals()
    local file = io.open(rental_file, "w")
    if file then
        file:write(minetest.write_json(rentals))
        file:close()
    end
end

-- Charger au démarrage
load_rentals()

-- Structure d'une location
local function create_rental(world, pos1, pos2, price_per_day, metadata)
    local rental = {
        id = minetest.pos_to_string(pos1) .. "_" .. os.time(),
        world = world,
        position = pos1,
        size = {
            x = math.abs(pos2.x - pos1.x) + 1,
            y = math.abs(pos2.y - pos1.y) + 1,
            z = math.abs(pos2.z - pos1.z) + 1
        },
        owner = minetest.settings:get("mahala_wallet_address") or "",
        tenant = nil,
        price_per_day = price_per_day,
        contract_end = 0,
        metadata = metadata or {}
    }
    
    -- Protéger la zone
    minetest.set_node(pos1, {name = "mahala_rental:sign"})
    local meta = minetest.get_meta(pos1)
    meta:set_string("rental_id", rental.id)
    meta:set_string("infotext", "Emplacement à louer: " .. price_per_day .. " M/jour")
    
    rentals[rental.id] = rental
    save_rentals()
    
    return rental
end

-- Obtenir une location
local function get_rental(rental_id)
    return rentals[rental_id]
end

-- Louer un emplacement
local function rent_space(player_name, rental_id, duration_days)
    local rental = get_rental(rental_id)
    if not rental then
        return false, "Location introuvable"
    end
    
    if rental.tenant then
        return false, "Déjà loué"
    end
    
    -- Récupérer le wallet du joueur
    local player_data = minetest.get_player_meta(player_name)
    local wallet = player_data:get_string("mahala_wallet")
    if not wallet or wallet == "" then
        return false, "Aucun wallet configuré"
    end
    
    local total_cost = rental.price_per_day * duration_days
    
    -- Vérifier la balance (simplifié, utiliser la fonction du mod teleport)
    -- Dans une vraie implémentation, on appellerait l'API
    
    -- Transaction blockchain
    local tx_success = true -- Simplifié pour l'instant
    
    if tx_success then
        rental.tenant = wallet
        rental.contract_end = os.time() + (duration_days * 86400)
        save_rentals()
        
        minetest.chat_send_player(
            player_name,
            "✅ Emplacement loué pour " .. duration_days .. " jours (-" .. total_cost .. " M)"
        )
        
        return true, rental
    else
        return false, "Transaction échouée"
    end
end

-- Vérifier les expirations
local function check_rental_expirations()
    local now = os.time()
    
    for id, rental in pairs(rentals) do
        if rental.tenant and rental.contract_end < now then
            -- Contrat expiré
            rental.tenant = nil
            rental.contract_end = 0
            
            -- Replacer le panneau
            local meta = minetest.get_meta(rental.position)
            meta:set_string("infotext", "Emplacement à louer: " .. rental.price_per_day .. " M/jour")
            
            save_rentals()
        end
    end
end

-- Vérifier toutes les heures
minetest.register_globalstep(function(dtime)
    if os.time() % 3600 == 0 then
        check_rental_expirations()
    end
end)

-- Node pour le panneau de location
minetest.register_node("mahala_rental:sign", {
    description = "Panneau Location Mahala",
    tiles = {"mahala_rental_sign.png"},
    groups = {dig_immediate = 2},
    paramtype = "light",
    drawtype = "nodebox",
    node_box = {
        type = "fixed",
        fixed = {-0.5, -0.5, 0.4, 0.5, 0.5, 0.5}
    }
})

-- Interaction avec le panneau
minetest.register_on_punchnode(function(pos, node, puncher, pointed_thing)
    if node.name == "mahala_rental:sign" then
        local meta = minetest.get_meta(pos)
        local rental_id = meta:get_string("rental_id")
        
        if rental_id and rental_id ~= "" then
            local rental = get_rental(rental_id)
            if rental then
                show_rental_interface(puncher:get_player_name(), rental_id)
            end
        end
    end
end)

-- Interface de location
local function show_rental_interface(player_name, rental_id)
    local rental = get_rental(rental_id)
    if not rental then
        return
    end
    
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
    end
    
    minetest.show_formspec(player_name, "mahala:rental_" .. rental_id, formspec)
end

-- Gérer les actions
minetest.register_on_player_receive_fields(function(player, formname, fields)
    if string.find(formname, "mahala:rental_") then
        local rental_id = string.sub(formname, 15) -- Enlever "mahala:rental_"
        
        if fields.rent then
            local duration = tonumber(fields.duration) or 30
            local success, result = rent_space(player:get_player_name(), rental_id, duration)
            if not success then
                minetest.chat_send_player(player:get_player_name(), "❌ " .. result)
            end
        end
        
        return true
    end
end)

-- Commande admin pour créer une zone locative
minetest.register_chatcommand("create_rental", {
    params = "<price_per_day>",
    description = "Créer une zone locative (admin uniquement)",
    privs = {server = true},
    func = function(name, param)
        local price = tonumber(param) or 10.0
        local player = minetest.get_player_by_name(name)
        if not player then
            return false, "Joueur introuvable"
        end
        
        local pos = player:get_pos()
        local pos1 = {x = math.floor(pos.x - 5), y = math.floor(pos.y), z = math.floor(pos.z - 5)}
        local pos2 = {x = math.floor(pos.x + 5), y = math.floor(pos.y + 5), z = math.floor(pos.z + 5)}
        
        local world = minetest.settings:get("world_name") or "Monde"
        local rental = create_rental(world, pos1, pos2, price, {})
        
        return true, "Zone locative créée: " .. rental.id
    end
})

print("[Mahala Rental] Mod chargé")

