<?php
/**
 * Widget wallet Mahala pour dashboard
 *
 * @package    Mahala
 */

/**
 * Class pour le widget wallet
 */
class mahala_wallet_box extends ModeleBoxes
{
    public $boxcode = "mahala_wallet";
    public $boximg = "mahala@mahala";
    public $boxlabel = "MahalaWallet";
    public $depends = array("mahala");
    public $hidden = 0;
    public $info_box_head = array();
    public $info_box_contents = array();

    /**
     * Constructor
     */
    public function __construct()
    {
        global $langs;
        $langs->load("mahala@mahala");
        $this->boxlabel = $langs->trans("MahalaWallet");
    }

    /**
     * Load data into info_box_contents array
     *
     * @param int $max Maximum number of records to load
     * @return void
     */
    public function loadBox($max = 5)
    {
        global $conf, $user, $langs, $db;

        $this->max = $max;
        require_once DOL_DOCUMENT_ROOT.'/custom/mahala/class/wallet.class.php';

        $langs->load("mahala@mahala");

        // Récupérer les wallets de l'utilisateur
        $wallet = new MahalaWallet($db);
        $wallets = array();

        // Pour l'instant, on affiche un message
        $this->info_box_head = array(
            array(
                'text' => $langs->trans("MahalaWallets"),
                'logo' => 'mahala@mahala'
            )
        );

        $this->info_box_contents[0][0] = array(
            'td' => 'class="tdoverflowmax200"',
            'text' => $langs->trans("NoWalletYet"),
        );
    }

    /**
     * Method to show box
     *
     * @param array $head Array with properties of box title
     * @param array $contents Array with properties of box lines
     * @param int $nooutput No print, only return string
     * @param string $footer Optional footer
     * @return string
     */
    public function showBox($head = null, $contents = null, $nooutput = 0, $footer = "")
    {
        return parent::showBox($this->info_box_head, $this->info_box_contents, $nooutput, $footer);
    }
}

