<?php
/**
 * Module Mahala pour Dolibarr
 *
 * Gestion des wallets Mahala et June dans Dolibarr
 * Intégration avec le bridge et le métaverse
 *
 * @package    Mahala
 * @author     Mahala Team
 * @copyright  2024
 * @license    MIT
 */

/**
 * Classe principale du module Mahala
 */
class modMahala extends DolibarrModules
{
    /**
     * Constructor
     *
     * @param DoliDB $db Database handler
     */
    public function __construct($db)
    {
        $this->db = $db;
        $this->editor_name = 'Mahala Team';
        $this->editor_url = 'https://mahala.org';
        $this->numero = 500000;
        $this->rights_class = 'mahala';
        $this->family = "financial";
        $this->module_position = '80';
        $this->name = 'Mahala';
        $this->description = "Gestion des wallets Mahala et June, intégration bridge et métaverse";
        $this->descriptionlong = "Module pour gérer les wallets Mahala et June dans Dolibarr, "
            . "effectuer des échanges via le bridge, et intégrer avec le métaverse Luanti.";
        $this->version = '1.0.0';
        $this->const_name = 'MAIN_MODULE_MAHALA';
        $this->picto = 'mahala@mahala';
        $this->module_parts = array(
            'hooks' => array('invoicecard', 'propalcard', 'ordercard', 'thirdpartycard'),
            'triggers' => 1,
            'login' => 0,
            'substitutions' => 0,
            'menus' => 0,
            'theme' => 0,
            'tabs' => array('thirdparty' => 'mahala:wallet'),
            'permissions' => 1,
        );
        $this->dirs = array();
        $this->config_page_url = array("setup.php@mahala");
        $this->depends = array();
        $this->requiredby = array();
        $this->langfiles = array("mahala@mahala");
        $this->phpmin = array(7, 4);
        $this->need_dolibarr_version = array(17, 0);
        $this->warnings_activation = array();
        $this->warnings_activation_ext = array();
    }

    /**
     * Function called when module is enabled
     *
     * @param string $options Options when enabling module
     * @return int 1 if OK, 0 if KO
     */
    public function init($options = '')
    {
        $result = $this->_load_tables('/mahala/sql/');
        if ($result < 0) {
            return -1;
        }

        return $this->_init($this->db, $options);
    }

    /**
     * Function called when module is disabled
     *
     * @param string $options Options when disabling module
     * @return int 1 if OK, 0 if KO
     */
    public function remove($options = '')
    {
        return $this->_remove($this->db, $options);
    }
}

