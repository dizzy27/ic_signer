use crate::crypto::Hash256;
use hex::FromHex;
use k256::ecdsa::{signature, signature::DigestVerifier, Signature, VerifyingKey};
use sha3::{Digest, Keccak256, Sha3_256};

pub fn hexstr_to_vec(text: &str) -> Result<Vec<u8>, String> {
    let data = match Vec::from_hex(text) {
        Ok(vec) => vec,
        Err(_) => return Err("Failed to encode from hex string".to_string()),
    };

    let vec: Vec<u8> = data;
    Ok(vec)
}

pub fn vec8_to_hexstr(data: &Vec<u8>) -> String {
    hex::encode(data)
}

pub fn hash_sha256(data: &Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha3_256::new();

    // write input message
    hasher.update(data);

    // read hash digest
    hasher.finalize().to_vec()
}

pub fn hash_keccak256(data: &Vec<u8>) -> Vec<u8> {
    let mut hasher = Keccak256::new();

    // write input message
    hasher.update(data);

    // read hash digest
    hasher.finalize().to_vec()
}

pub fn verify_signature(
    msg_hash: &[u8],
    sig_bytes: &[u8],
    pubkey_bytes: &[u8],
) -> bool {
    // let message_hash = hash_keccak256(&message.to_vec());
    let signature: Signature = signature::Signature::from_bytes(sig_bytes)
        .expect("Response is not a valid signature");
    let verifying_key = VerifyingKey::from_sec1_bytes(pubkey_bytes)
        .expect("Response is not a valid public key");
    let digest = Hash256::<Sha3_256>::try_from(msg_hash.as_ref())
        .expect("Message is not a valid SHA256");
    verifying_key.verify_digest(digest, &signature).is_ok()
}