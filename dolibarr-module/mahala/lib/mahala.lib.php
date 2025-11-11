<?php
/**
 * Fonctions libres pour Mahala
 *
 * @package    Mahala
 */

/**
 * Afficher le widget wallet dans le dashboard
 *
 * @param int $socid ID du tiers
 * @return void
 */
function mahala_show_wallet_widget($socid)
{
    global $db, $langs;

    require_once DOL_DOCUMENT_ROOT.'/custom/mahala/class/wallet.class.php';

    $wallet = new MahalaWallet($db);
    $wallets = $wallet->getWalletsBySoc($socid);

    if (is_array($wallets) && count($wallets) > 0) {
        echo '<div class="fichecenter">';
        echo '<div class="fichehalfleft">';
        echo '<table class="noborder centpercent">';
        echo '<tr class="liste_titre">';
        echo '<th colspan="2">'.$langs->trans("MahalaWallets").'</th>';
        echo '</tr>';

        foreach ($wallets as $w) {
            echo '<tr>';
            echo '<td>'.strtoupper($w->wallet_type).'</td>';
            echo '<td class="right">';
            echo '<strong>'.number_format($w->balance, 2, '.', ' ').'</strong> ';
            echo strtoupper($w->wallet_type);
            echo '</td>';
            echo '</tr>';
        }

        echo '</table>';
        echo '</div>';
        echo '</div>';
    }
}

/**
 * Convertir un montant Mahala en Euro (via bridge)
 *
 * @param float $mahala_amount Montant en Mahala
 * @return float|false Montant en Euro ou false si erreur
 */
function mahala_convert_to_euro($mahala_amount)
{
    require_once DOL_DOCUMENT_ROOT.'/custom/mahala/class/bridge_api.class.php';

    $bridge = new MahalaBridgeAPI();
    
    // Obtenir le taux via le bridge (en passant par June)
    // 1. Mahala → June
    $quote_mj = $bridge->getQuote('mahala_to_june', $mahala_amount);
    if (!$quote_mj) {
        return false;
    }
    
    // 2. June → Euro (taux fixe pour l'instant, à améliorer)
    // Pour l'instant, on utilise un taux approximatif
    // Dans une vraie implémentation, on récupérerait le taux depuis une API
    $june_to_euro_rate = 0.01; // 1 June = 0.01 EUR (exemple)
    
    $june_amount = $quote_mj['output'];
    $euro_amount = $june_amount * $june_to_euro_rate;
    
    return $euro_amount;
}

/**
 * Convertir un montant Euro en Mahala (via bridge)
 *
 * @param float $euro_amount Montant en Euro
 * @return float|false Montant en Mahala ou false si erreur
 */
function mahala_convert_from_euro($euro_amount)
{
    require_once DOL_DOCUMENT_ROOT.'/custom/mahala/class/bridge_api.class.php';

    $bridge = new MahalaBridgeAPI();
    
    // 1. Euro → June (taux fixe pour l'instant)
    $euro_to_june_rate = 100; // 1 EUR = 100 June (exemple)
    $june_amount = $euro_amount * $euro_to_june_rate;
    
    // 2. June → Mahala
    $quote_jm = $bridge->getQuote('june_to_mahala', $june_amount);
    if (!$quote_jm) {
        return false;
    }
    
    return $quote_jm['output'];
}

