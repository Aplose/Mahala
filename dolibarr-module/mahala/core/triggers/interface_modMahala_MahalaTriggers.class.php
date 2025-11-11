<?php
/**
 * Triggers Mahala
 *
 * @package    Mahala
 */

require_once DOL_DOCUMENT_ROOT.'/core/triggers/dolibarrtriggers.class.php';

/**
 * Class des triggers Mahala
 */
class InterfaceMahalaTriggers extends DolibarrTriggers
{
    /**
     * Constructor
     *
     * @param DoliDB $db Database handler
     */
    public function __construct($db)
    {
        $this->db = $db;
        $this->name = preg_replace('/^Interface/i', '', get_class($this));
        $this->family = "mahala";
        $this->description = "Triggers Mahala";
        $this->version = '1.0.0';
        $this->picto = 'mahala@mahala';
    }

    /**
     * Trigger appelé lors de la validation d'une facture
     *
     * @param string $action Action
     * @param Facture $object Facture
     * @param User $user User
     * @param Translate $langs Langs
     * @param Conf $conf Conf
     * @return int <0 si KO, 0 si rien fait, >0 si OK
     */
    public function doActions($action, $object, $user, $langs, $conf)
    {
        if ($action == 'BILL_VALIDATE') {
            // Si la facture est payée en Mahala, enregistrer la transaction
            if (!empty($object->array_options['options_payment_method']) 
                && $object->array_options['options_payment_method'] == 'mahala') {
                
                // TODO: Enregistrer la transaction Mahala
                dol_syslog("Facture payée en Mahala: ".$object->ref);
            }
        }

        return 0;
    }
}

