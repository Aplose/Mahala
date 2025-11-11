<?php
/**
 * Classe Wallet Mahala
 *
 * Gestion des wallets Mahala et June pour les tiers
 *
 * @package    Mahala
 */

require_once DOL_DOCUMENT_ROOT.'/core/class/commonobject.class.php';

/**
 * Classe Wallet
 */
class MahalaWallet extends CommonObject
{
    /**
     * @var int ID
     */
    public $id;

    /**
     * @var int ID du tiers
     */
    public $fk_soc;

    /**
     * @var string Type de wallet ('mahala' ou 'june')
     */
    public $wallet_type;

    /**
     * @var string Adresse du wallet
     */
    public $wallet_address;

    /**
     * @var float Balance
     */
    public $balance;

    /**
     * @var datetime Dernière synchronisation
     */
    public $last_sync;

    /**
     * @var int Synchronisation activée
     */
    public $sync_enabled;

    /**
     * Constructor
     *
     * @param DoliDB $db Database handler
     */
    public function __construct($db)
    {
        $this->db = $db;
        $this->table_element = 'mahala_wallet';
        $this->table_element_line = '';
    }

    /**
     * Créer un wallet
     *
     * @param User $user User qui crée
     * @return int <0 si KO, >0 si OK
     */
    public function create($user)
    {
        global $conf;

        $error = 0;

        $this->date_creation = dol_now();
        $this->fk_user_creat = $user->id;

        $sql = "INSERT INTO ".MAIN_DB_PREFIX."mahala_wallet (";
        $sql .= "fk_soc,";
        $sql .= "wallet_type,";
        $sql .= "wallet_address,";
        $sql .= "balance,";
        $sql .= "sync_enabled,";
        $sql .= "date_creation,";
        $sql .= "fk_user_creat";
        $sql .= ") VALUES (";
        $sql .= (int) $this->fk_soc.",";
        $sql .= "'".$this->db->escape($this->wallet_type)."',";
        $sql .= "'".$this->db->escape($this->wallet_address)."',";
        $sql .= (float) $this->balance.",";
        $sql .= (int) $this->sync_enabled.",";
        $sql .= "'".$this->db->idate($this->date_creation)."',";
        $sql .= (int) $this->fk_user_creat;
        $sql .= ")";

        $this->db->begin();

        dol_syslog(__METHOD__, LOG_DEBUG);
        $resql = $this->db->query($sql);
        if (!$resql) {
            $error++;
            $this->errors[] = "Error ".$this->db->lasterror();
        }

        if (!$error) {
            $this->id = $this->db->last_insert_id(MAIN_DB_PREFIX."mahala_wallet");
            $this->db->commit();
            return $this->id;
        } else {
            $this->db->rollback();
            return -1;
        }
    }

    /**
     * Charger un wallet
     *
     * @param int $id ID du wallet
     * @return int <0 si KO, >0 si OK
     */
    public function fetch($id)
    {
        $sql = "SELECT w.rowid, w.fk_soc, w.wallet_type, w.wallet_address,";
        $sql .= " w.balance, w.last_sync, w.sync_enabled";
        $sql .= " FROM ".MAIN_DB_PREFIX."mahala_wallet as w";
        $sql .= " WHERE w.rowid = ".(int) $id;

        dol_syslog(__METHOD__, LOG_DEBUG);
        $resql = $this->db->query($sql);
        if ($resql) {
            $obj = $this->db->fetch_object($resql);

            $this->id = $obj->rowid;
            $this->fk_soc = $obj->fk_soc;
            $this->wallet_type = $obj->wallet_type;
            $this->wallet_address = $obj->wallet_address;
            $this->balance = $obj->balance;
            $this->last_sync = $this->db->jdate($obj->last_sync);
            $this->sync_enabled = $obj->sync_enabled;

            return 1;
        } else {
            $this->error = "Error ".$this->db->lasterror();
            return -1;
        }
    }

    /**
     * Synchroniser la balance avec la blockchain
     *
     * @return int <0 si KO, >0 si OK
     */
    public function syncBalance()
    {
        global $conf;

        if (empty($conf->global->MAHALA_NODE_URL)) {
            $this->error = "Mahala node URL not configured";
            return -1;
        }

        $url = rtrim($conf->global->MAHALA_NODE_URL, '/');
        $api_url = $url."/blockchain/balance/".$this->wallet_address;

        // Appel API
        $ch = curl_init($api_url);
        curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
        curl_setopt($ch, CURLOPT_TIMEOUT, 10);
        $response = curl_exec($ch);
        $http_code = curl_getinfo($ch, CURLINFO_HTTP_CODE);
        curl_close($ch);

        if ($http_code == 200 && $response) {
            $data = json_decode($response, true);
            if (isset($data['balance'])) {
                $old_balance = $this->balance;
                $this->balance = (float) $data['balance'];
                $this->last_sync = dol_now();

                // Mettre à jour en base
                $sql = "UPDATE ".MAIN_DB_PREFIX."mahala_wallet SET";
                $sql .= " balance = ".(float) $this->balance.",";
                $sql .= " last_sync = '".$this->db->idate($this->last_sync)."'";
                $sql .= " WHERE rowid = ".(int) $this->id;

                $this->db->query($sql);

                return 1;
            }
        }

        $this->error = "Failed to sync balance";
        return -1;
    }

    /**
     * Obtenir les wallets d'un tiers
     *
     * @param int $socid ID du tiers
     * @return array|int Array de wallets ou <0 si KO
     */
    public function getWalletsBySoc($socid)
    {
        $sql = "SELECT rowid FROM ".MAIN_DB_PREFIX."mahala_wallet";
        $sql .= " WHERE fk_soc = ".(int) $socid;

        $wallets = array();
        $resql = $this->db->query($sql);
        if ($resql) {
            while ($obj = $this->db->fetch_object($resql)) {
                $wallet = new MahalaWallet($this->db);
                $wallet->fetch($obj->rowid);
                $wallets[] = $wallet;
            }
            return $wallets;
        } else {
            $this->error = "Error ".$this->db->lasterror();
            return -1;
        }
    }
}

