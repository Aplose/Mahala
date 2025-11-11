<?php
/**
 * Classe API Bridge Mahala ↔ June
 *
 * Interface avec le service bridge pour les échanges
 *
 * @package    Mahala
 */

require_once DOL_DOCUMENT_ROOT.'/core/class/commonobject.class.php';

/**
 * Classe BridgeAPI
 */
class MahalaBridgeAPI
{
    /**
     * @var string URL du bridge
     */
    private $bridge_url;

    /**
     * Constructor
     */
    public function __construct()
    {
        global $conf;
        $this->bridge_url = !empty($conf->global->MAHALA_BRIDGE_URL) 
            ? rtrim($conf->global->MAHALA_BRIDGE_URL, '/')
            : 'http://localhost:8081';
    }

    /**
     * Obtenir les statistiques du bridge
     *
     * @return array|false Statistiques ou false si erreur
     */
    public function getStats()
    {
        $url = $this->bridge_url."/bridge/stats";
        
        $ch = curl_init($url);
        curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
        curl_setopt($ch, CURLOPT_TIMEOUT, 10);
        $response = curl_exec($ch);
        $http_code = curl_getinfo($ch, CURLINFO_HTTP_CODE);
        curl_close($ch);

        if ($http_code == 200 && $response) {
            return json_decode($response, true);
        }

        return false;
    }

    /**
     * Obtenir un devis pour un échange
     *
     * @param string $direction 'mahala_to_june' ou 'june_to_mahala'
     * @param float $amount Montant
     * @return array|false Quote ou false si erreur
     */
    public function getQuote($direction, $amount)
    {
        $url = $this->bridge_url."/bridge/quote";
        
        $data = json_encode(array(
            'direction' => $direction,
            'amount' => $amount
        ));

        $ch = curl_init($url);
        curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
        curl_setopt($ch, CURLOPT_POST, true);
        curl_setopt($ch, CURLOPT_POSTFIELDS, $data);
        curl_setopt($ch, CURLOPT_HTTPHEADER, array(
            'Content-Type: application/json',
            'Content-Length: ' . strlen($data)
        ));
        curl_setopt($ch, CURLOPT_TIMEOUT, 10);
        $response = curl_exec($ch);
        $http_code = curl_getinfo($ch, CURLINFO_HTTP_CODE);
        curl_close($ch);

        if ($http_code == 200 && $response) {
            return json_decode($response, true);
        }

        return false;
    }

    /**
     * Exécuter un échange
     *
     * @param string $direction Direction
     * @param float $amount Montant
     * @param string $wallet_address Adresse du wallet
     * @return array|false Résultat ou false si erreur
     */
    public function executeExchange($direction, $amount, $wallet_address)
    {
        $url = $this->bridge_url."/bridge/exchange";
        
        $data = json_encode(array(
            'direction' => $direction,
            'amount' => $amount,
            'wallet_address' => $wallet_address
        ));

        $ch = curl_init($url);
        curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
        curl_setopt($ch, CURLOPT_POST, true);
        curl_setopt($ch, CURLOPT_POSTFIELDS, $data);
        curl_setopt($ch, CURLOPT_HTTPHEADER, array(
            'Content-Type: application/json',
            'Content-Length: ' . strlen($data)
        ));
        curl_setopt($ch, CURLOPT_TIMEOUT, 30);
        $response = curl_exec($ch);
        $http_code = curl_getinfo($ch, CURLINFO_HTTP_CODE);
        curl_close($ch);

        if ($http_code == 200 && $response) {
            return json_decode($response, true);
        }

        return false;
    }

    /**
     * Obtenir le taux de change actuel
     *
     * @param string $direction Direction
     * @param float $amount Montant (pour calculer le taux)
     * @return float|false Taux ou false si erreur
     */
    public function getExchangeRate($direction, $amount = 1.0)
    {
        $quote = $this->getQuote($direction, $amount);
        if ($quote && isset($quote['rate'])) {
            return (float) $quote['rate'];
        }
        return false;
    }
}

