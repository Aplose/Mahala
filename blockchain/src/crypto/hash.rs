//! Fonctions de hachage utilisant Blake3 pour performance optimale

use blake3;
use crate::Hash;

/// Hasher des données en utilisant Blake3
pub fn hash_data(data: &[u8]) -> Hash {
    blake3::hash(data).into()
}

/// Convertir un hash en string hexadécimale
pub fn hash_to_string(hash: &Hash) -> String {
    hex::encode(hash)
}

/// Parser un hash depuis une string hexadécimale
pub fn hash_from_string(s: &str) -> Result<Hash, String> {
    let bytes = hex::decode(s)
        .map_err(|e| format!("Invalid hex: {}", e))?;
    
    if bytes.len() != 32 {
        return Err(format!("Hash must be 32 bytes, got {}", bytes.len()));
    }
    
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&bytes);
    Ok(hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_data() {
        let data = b"test data";
        let hash = hash_data(data);
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_hash_string_conversion() {
        let data = b"test";
        let hash = hash_data(data);
        let s = hash_to_string(&hash);
        let parsed = hash_from_string(&s).unwrap();
        assert_eq!(hash, parsed);
    }
}


