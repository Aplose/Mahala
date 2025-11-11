-- Mod Mahala NFT
-- Affichage et interaction avec les NFT dans Luanti

local http = minetest.request_http_api()
if not http then
    error("Mahala NFT requires HTTP API")
end

local MAHALA_API = minetest.settings:get("mahala_node_url") or "http://localhost:8080"
local NFT_CONTRACT_ADDRESS = minetest.settings:get("nft_contract_address") or ""

-- Récupérer un NFT depuis la blockchain
local function fetch_nft_from_blockchain(nft_id)
    -- TODO: Implémenter l'appel réel à l'API NFT
    -- Pour l'instant, on simule
    return {
        id = nft_id,
        owner = "abc123...",
        creator = "def456...",
        metadata = {
            name = "NFT Example",
            description = "Un NFT d'exemple",
            nft_type = "Art",
            media_url = "https://example.com/image.png",
            thumbnail_url = "https://example.com/thumb.png"
        }
    }
end

-- Afficher un NFT dans le monde
local function display_nft_in_world(pos, nft_id)
    local nft = fetch_nft_from_blockchain(nft_id)
    if not nft then
        return false
    end
    
    -- Créer un cadre pour le NFT
    minetest.set_node(pos, {name = "mahala_nft:frame"})
    
    local meta = minetest.get_meta(pos)
    meta:set_string("nft_id", nft_id)
    meta:set_string("nft_name", nft.metadata.name)
    meta:set_string("owner", nft.owner)
    meta:set_string("creator", nft.creator)
    meta:set_string("infotext", nft.metadata.name .. " (NFT)")
    
    return true
end

-- Node pour le cadre NFT
minetest.register_node("mahala_nft:frame", {
    description = "Cadre NFT Mahala",
    tiles = {
        "mahala_nft_frame_top.png",
        "mahala_nft_frame_bottom.png",
        "mahala_nft_frame_side.png"
    },
    groups = {dig_immediate = 2},
    paramtype = "light",
    drawtype = "nodebox",
    node_box = {
        type = "fixed",
        fixed = {
            {-0.5, -0.5, -0.5, 0.5, 0.5, 0.5}
        }
    },
    on_punch = function(pos, node, puncher, pointed_thing)
        local meta = minetest.get_meta(pos)
        local nft_id = meta:get_string("nft_id")
        
        if nft_id and nft_id ~= "" then
            local nft = fetch_nft_from_blockchain(nft_id)
            if nft then
                show_nft_info(puncher:get_player_name(), nft)
            end
        end
    end
})

-- Afficher les informations d'un NFT
local function show_nft_info(player_name, nft)
    local formspec = "size[10,8]" ..
        "label[0.5,0.5;" .. minetest.formspec_escape(nft.metadata.name) .. "]" ..
        "image[0.5,1;3,3;" .. nft.metadata.thumbnail_url .. "]" ..
        "label[4,1;Créateur: " .. string.sub(nft.creator, 1, 20) .. "...]" ..
        "label[4,1.5;Propriétaire: " .. string.sub(nft.owner, 1, 20) .. "...]" ..
        "label[4,2;Type: " .. nft.metadata.nft_type .. "]" ..
        "textarea[0.5,4.5;9,3;description;;" ..
            minetest.formspec_escape(nft.metadata.description) .. "]"
    
    -- Vérifier si le joueur est le propriétaire
    local player_data = minetest.get_player_meta(player_name)
    local player_wallet = player_data:get_string("mahala_wallet")
    
    if player_wallet == nft.owner then
        formspec = formspec ..
            "button[1,7;3,0.8;sell;Mettre en vente]" ..
            "button[6,7;3,0.8;transfer;Transférer]"
    end
    
    minetest.show_formspec(player_name, "mahala:nft_" .. nft.id, formspec)
end

-- Commande pour placer un NFT
minetest.register_chatcommand("place_nft", {
    params = "<nft_id>",
    description = "Placer un NFT dans le monde",
    func = function(name, param)
        if param == "" then
            return false, "Usage: /place_nft <nft_id>"
        end
        
        local player = minetest.get_player_by_name(name)
        if not player then
            return false, "Joueur introuvable"
        end
        
        local pos = player:get_pos()
        local place_pos = {
            x = math.floor(pos.x),
            y = math.floor(pos.y) + 1,
            z = math.floor(pos.z)
        }
        
        if display_nft_in_world(place_pos, param) then
            return true, "NFT placé"
        else
            return false, "Erreur lors du placement du NFT"
        end
    end
})

-- Commande pour lister ses NFT
minetest.register_chatcommand("my_nfts", {
    description = "Lister vos NFT",
    func = function(name, param)
        local player_data = minetest.get_player_meta(name)
        local wallet = player_data:get_string("mahala_wallet")
        
        if not wallet or wallet == "" then
            return false, "Aucun wallet configuré"
        end
        
        -- TODO: Appeler l'API pour récupérer les NFT du wallet
        return true, "Fonctionnalité à implémenter"
    end
})

print("[Mahala NFT] Mod chargé")

