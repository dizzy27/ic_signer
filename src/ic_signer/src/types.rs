use crate::crypto::Hash256;
use crate::utils::{hexstr_to_vec, vec8_to_hexstr};
use base64;
use k256::ecdsa::{recoverable, signature::DigestSigner, SigningKey};
use serde::Serialize;
use sha3::{Keccak256, Sha3_256};

const ECDSA_PRIVKEY_LEN: usize = 32;

#[derive(Copy, Clone)]
pub enum HashAlgorithm {
    // SHA2_256,
    SHA3_256,
    Keccak256,
}

impl Default for HashAlgorithm {
    fn default() -> Self {
        HashAlgorithm::Keccak256
    }
}

pub trait PrivateKey {
    fn to_string(&self) -> String;
    fn to_vec8(&self) -> Vec<u8>;
    fn len(&self) -> usize;
    fn sign(&self, msg_hash: &Vec<u8>, hash_algo: HashAlgorithm) -> Result<Vec<u8>, String>;
    fn to_pubkey(&self) -> Result<Vec<u8>, String>;
}

pub trait PublicKey {
    fn to_string(&self) -> String;
    fn from_vec8(&self, data: &Vec<u8>) -> ();
    fn to_vec8(&self) -> String;
    fn len(&self) -> usize;
}

#[derive(Serialize, Debug)]
pub struct Bundle {
    pub digest: Vec<u8>,
    pub publickey: Vec<u8>,
    pub signature: Vec<u8>,
}

#[derive(Debug)]
pub struct ECDSAPrivateKey {
    data: Vec<u8>,
}

impl ECDSAPrivateKey {
    pub fn from_string(hex_string: &str) -> Result<ECDSAPrivateKey, String> {
        if hex_string.len() != ECDSA_PRIVKEY_LEN * 2 {
            return Err("ECDSA private key string's length error".to_string());
        }
        match hexstr_to_vec(hex_string) {
            Ok(res) => {
                let key = ECDSAPrivateKey { data: res };
                Ok(key)
            }
            Err(e) => Err(e),
        }
    }

    pub fn from_vec8(vec: &Vec<u8>) -> Result<ECDSAPrivateKey, String> {
        if vec.len() != ECDSA_PRIVKEY_LEN {
            return Err("ECDSA private key string's length error".to_string());
        }
        let key = ECDSAPrivateKey { data: vec.clone() };
        Ok(key)
    }
}

impl PrivateKey for ECDSAPrivateKey {
    fn to_string(&self) -> String {
        vec8_to_hexstr(&self.data)
    }

    fn to_vec8(&self) -> Vec<u8> {
        self.data.clone()
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn sign(&self, msg_hash: &Vec<u8>, hash_algo: HashAlgorithm) -> Result<Vec<u8>, String> {
        let signing_key = match SigningKey::from_bytes(&self.data) {
            Ok(key) => key,
            Err(_) => return Err("Get signing key failed".to_string()),
        };
        // let rsv: recoverable::Signature = signing_key.sign(&message_u8);
        type HashSha3_256 = Hash256<Sha3_256>;
        type HashKeccak256 = Hash256<Keccak256>;
        let rsv: recoverable::Signature = match hash_algo {
            HashAlgorithm::SHA3_256 => {
                let digest = HashSha3_256::try_from(msg_hash.as_ref())?;
                DigestSigner::sign_digest(&signing_key, digest)
            }
            HashAlgorithm::Keccak256 => {
                let digest = HashKeccak256::try_from(msg_hash.as_ref())?;
                DigestSigner::sign_digest(&signing_key, digest)
            }
        };
        // let signature: Vec<u8> = rsv.as_ref()[..64].to_vec();
        let signature: Vec<u8> = rsv.as_ref().to_vec();

        // let id: u8 = rsv.recovery_id().into();
        // println!("signature: {}", vec8_to_hexstr(&rsv.as_ref().to_vec()));
        // println!("id: {}", id);
        Ok(signature)
    }

    fn to_pubkey(&self) -> Result<Vec<u8>, String> {
        let signing_key = match SigningKey::from_bytes(&self.data) {
            Ok(key) => key,
            Err(_) => return Err("Get signing key failed".to_string()),
        };
        let verify_key = signing_key.verifying_key();
        let pubkey_info = k256::PublicKey::from(verify_key).to_string();
        let end = pubkey_info.len() - 24 - 2; // remove -----END PUBLIC KEY-----
        let pubkey_str = &pubkey_info[27..end].replace("\n", ""); // remove -----BEGIN PUBLIC KEY-----
        let pubkey = match base64::decode(pubkey_str) {
            Ok(bytes) => bytes,
            Err(_) => return Err("Decode pubkey_str failed".to_string()),
        };
        // println!("pubkey: {:?}", vec8_to_hexstr(&pubkey[..23].to_vec()));
        // println!("pubkey: {:?}", vec8_to_hexstr(&pubkey[23..].to_vec()));
        Ok(pubkey[23..].to_vec())
    }
}
// MFYwEAYHKoZIzj0CAQYFK4EEAAoDQgAEWn83kD4nNdAJEhVemPEwJeCwldjT/bhCW5gbK2+9TApxBBXxu40HwMEZP/jrOYr4Dhuat8PnkISyo41zoOd0Vg==
// 045A7F37903E2735D00912155E98F13025E0B095D8D3FDB8425B981B2B6FBD4C0A710415F1BB8D07C0C1193FF8EB398AF80E1B9AB7C3E79084B2A38D73A0E77456
// 3056301006072a8648ce3d020106052b8104000a034200045a7f37903e2735d00912155e98f13025e0b095d8d3fdb8425b981b2b6fbd4c0a710415f1bb8d07c0c1193ff8eb398af80e1b9ab7c3e79084b2a38d73a0e77456

// #[derive(Debug)]
// struct ECDSAPublicKey {
//     pub canister_id: Option<CanisterId>,
//     pub derivation_path: Vec<Vec<u8>>,
//     pub key_id: EcdsaKeyId,
// }