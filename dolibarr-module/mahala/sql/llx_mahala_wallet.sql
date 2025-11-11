-- Table pour stocker les wallets Mahala et June des tiers
CREATE TABLE IF NOT EXISTS llx_mahala_wallet (
    rowid INTEGER AUTO_INCREMENT PRIMARY KEY,
    fk_soc INTEGER NOT NULL,
    wallet_type VARCHAR(10) NOT NULL DEFAULT 'mahala', -- 'mahala' ou 'june'
    wallet_address VARCHAR(128) NOT NULL,
    balance DECIMAL(20, 8) DEFAULT 0.00000000,
    last_sync DATETIME,
    sync_enabled TINYINT(1) DEFAULT 1,
    date_creation DATETIME NOT NULL,
    tms TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    fk_user_creat INTEGER,
    fk_user_modif INTEGER,
    UNIQUE KEY uk_wallet_soc_type (fk_soc, wallet_type),
    KEY idx_wallet_address (wallet_address)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Table pour les transactions Mahala
CREATE TABLE IF NOT EXISTS llx_mahala_transaction (
    rowid INTEGER AUTO_INCREMENT PRIMARY KEY,
    fk_soc INTEGER,
    tx_hash VARCHAR(128) NOT NULL UNIQUE,
    tx_type VARCHAR(20) NOT NULL, -- 'transfer', 'du', 'exchange', etc.
    from_address VARCHAR(128),
    to_address VARCHAR(128),
    amount DECIMAL(20, 8) NOT NULL,
    fee DECIMAL(20, 8) DEFAULT 0.00000000,
    currency VARCHAR(10) NOT NULL DEFAULT 'mahala', -- 'mahala' ou 'june'
    block_height INTEGER,
    confirmed TINYINT(1) DEFAULT 0,
    date_creation DATETIME NOT NULL,
    tms TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    KEY idx_tx_hash (tx_hash),
    KEY idx_fk_soc (fk_soc),
    KEY idx_confirmed (confirmed)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Table pour les échanges via le bridge
CREATE TABLE IF NOT EXISTS llx_mahala_exchange (
    rowid INTEGER AUTO_INCREMENT PRIMARY KEY,
    fk_soc INTEGER NOT NULL,
    direction VARCHAR(20) NOT NULL, -- 'mahala_to_june' ou 'june_to_mahala'
    input_amount DECIMAL(20, 8) NOT NULL,
    output_amount DECIMAL(20, 8) NOT NULL,
    fee DECIMAL(20, 8) NOT NULL,
    rate DECIMAL(20, 8) NOT NULL,
    tx_hash VARCHAR(128),
    status VARCHAR(20) DEFAULT 'pending', -- 'pending', 'completed', 'failed'
    date_creation DATETIME NOT NULL,
    date_processed DATETIME,
    tms TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    KEY idx_fk_soc (fk_soc),
    KEY idx_status (status)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Table pour les boutiques virtuelles (métaverse)
CREATE TABLE IF NOT EXISTS llx_mahala_virtual_shop (
    rowid INTEGER AUTO_INCREMENT PRIMARY KEY,
    fk_soc INTEGER NOT NULL,
    shop_name VARCHAR(255) NOT NULL,
    luanti_world VARCHAR(100),
    luanti_position VARCHAR(100),
    virtual_shop_id VARCHAR(128),
    rental_id VARCHAR(128),
    active TINYINT(1) DEFAULT 1,
    date_creation DATETIME NOT NULL,
    tms TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    KEY idx_fk_soc (fk_soc),
    KEY idx_active (active)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

