# API Mahala

Documentation des APIs REST pour Mahala.

## Full Node API

Base URL: `http://node.mahala.org:8080`

### Health Check

```http
GET /health
```

**Response:**
```json
{
  "status": "ok",
  "service": "mahala-full-node"
}
```

### Blockchain

#### Obtenir la hauteur

```http
GET /blockchain/height
```

**Response:**
```json
{
  "height": 1234
}
```

#### Obtenir le dernier bloc

```http
GET /blockchain/last_block
```

**Response:**
```json
{
  "header": {
    "height": 1234,
    "previous_hash": "...",
    "merkle_root": "...",
    "timestamp": 1234567890,
    "validator": "...",
    "version": 1
  },
  "transactions": [...],
  "validator_signatures": [...]
}
```

#### Obtenir un bloc par hauteur

```http
GET /blockchain/block/{height}
```

#### Obtenir la balance d'une adresse

```http
GET /blockchain/balance/{address}
```

**Response:**
```json
{
  "address": "abc123...",
  "balance": 1000.5
}
```

### Transactions

#### Soumettre une transaction

```http
POST /transaction/submit
Content-Type: application/json

{
  "transaction": {
    "from": "...",
    "to": "...",
    "amount": 100.0,
    "fee": 0.1,
    "timestamp": 1234567890,
    "metadata": {
      "transaction_type": "Transfer"
    },
    "signature": "..."
  }
}
```

**Response:**
```json
{
  "status": "accepted",
  "tx_hash": "abc123..."
}
```

### Mempool

#### Obtenir la taille du mempool

```http
GET /mempool/size
```

**Response:**
```json
{
  "size": 150
}
```

## Bridge API

Base URL: `http://bridge.mahala.org:8081`

### Health Check

```http
GET /health
```

### Statistiques

```http
GET /bridge/stats
```

**Response:**
```json
{
  "reserves": {
    "june": 10000.0,
    "mahala": 10000.0
  },
  "market_maker": {
    "june_reserve": 10000.0,
    "mahala_reserve": 10000.0,
    "total_liquidity": 100000000.0,
    "fee_percentage": 0.1
  }
}
```

### Obtenir un devis

```http
POST /bridge/quote
Content-Type: application/json

{
  "direction": "mahala_to_june",
  "amount": 100.0
}
```

**Response:**
```json
{
  "input": 100.0,
  "output": 99.9,
  "fee": 0.1,
  "rate": 0.999
}
```

### Exécuter un échange

```http
POST /bridge/exchange
Content-Type: application/json

{
  "direction": "mahala_to_june",
  "amount": 100.0,
  "wallet_address": "abc123..."
}
```

**Response:**
```json
{
  "tx_hash": "0xabc123...",
  "amount_received": 99.9,
  "fee": 0.1
}
```

## Codes d'erreur

- `200` : Succès
- `400` : Requête invalide
- `404` : Ressource non trouvée
- `500` : Erreur serveur

## Authentification

Pour l'instant, les APIs sont publiques. L'authentification sera ajoutée dans une version future.

## Rate Limiting

Les APIs ont des limites de débit :
- Full Node : 100 requêtes/minute par IP
- Bridge : 50 requêtes/minute par IP

## Exemples

### cURL

```bash
# Obtenir la balance
curl http://node.mahala.org:8080/blockchain/balance/abc123...

# Soumettre une transaction
curl -X POST http://node.mahala.org:8080/transaction/submit \
  -H "Content-Type: application/json" \
  -d '{"transaction": {...}}'

# Obtenir un devis
curl -X POST http://bridge.mahala.org:8081/bridge/quote \
  -H "Content-Type: application/json" \
  -d '{"direction": "mahala_to_june", "amount": 100.0}'
```

### JavaScript

```javascript
// Obtenir la balance
const response = await fetch('http://node.mahala.org:8080/blockchain/balance/abc123...');
const data = await response.json();
console.log(data.balance);

// Soumettre une transaction
const txResponse = await fetch('http://node.mahala.org:8080/transaction/submit', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({ transaction: {...} })
});
```

