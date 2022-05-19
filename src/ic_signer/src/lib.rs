mod crypto;
mod types;
mod utils;

use types::{Bundle, ECDSAPrivateKey, HashAlgorithm, PrivateKey};
use utils::{hash_keccak256, hash_sha256, hexstr_to_vec, vec8_to_hexstr, verify_signature};
// use k256::sha2::{Sha256, Sha512, Digest};

// use ic_cdk::api::call::CallResult;
use ic_cdk::export::candid::{CandidType, Deserialize, Nat, Func};

#[derive(CandidType, Deserialize)]
pub struct HttpHeader(String, String);

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct Token {
    key: String,
    content_encoding: String,
    index: Nat,
    sha256: Option<u8>,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct CallbackStrategy {
    /// The callback function to be called to continue the stream.
    pub callback: Func,
    /// The token to pass to the function.
    pub token: Token,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub enum StreamingStrategy {
    /// A callback-based streaming strategy, where a callback function is provided for continuing the stream.
    Callback(CallbackStrategy),
}

#[derive(CandidType, Deserialize)]
pub struct HttpResponse {
    pub status_code: u16,
    pub body: Vec<u8>,
    pub headers: Vec<HttpHeader>,
    pub streaming_strategy: Option<StreamingStrategy>,
    pub upgrade: Option<bool>,
}

#[derive(CandidType, Deserialize)]
struct HttpRequest {
    pub url: String,
    pub method: String,
    pub body: Option<Vec<u8>>,
    pub headers: Vec<HttpHeader>,
}

// curl http://localhost:8000/?canisterId=rrkah-fqaaa-aaaaa-aaaaq-cai
#[ic_cdk_macros::query]
fn http_request(request: HttpRequest) -> HttpResponse {
    let status_code = 200;
    HttpResponse {
        body: test(),
        headers: vec![],
        status_code,
        streaming_strategy: None,
        upgrade: Some(false),
    }
}

fn sign(message: &str, privkey: impl PrivateKey) -> Result<Bundle, String> {
    let hash_algo = HashAlgorithm::Keccak256;
    let message_u8 = hexstr_to_vec(&hex::encode(message))?;
    let msg_hash = hash_keccak256(&message_u8);

    let sig = privkey.sign(&msg_hash, hash_algo)?;
    let pubkey = privkey.to_pubkey()?;

    let verified = verify_signature(&msg_hash, &sig, &pubkey);
    if verified {
        Ok(Bundle {
            message: message_u8,
            publickey: pubkey,
            signature: sig,
        })
    } else {
        return Err("Signature verified failed".to_string());
    }
}

#[ic_cdk_macros::query]
fn test() -> Vec<u8> {
    let mut privkey_str = "6a73b985cfd0142ba4be36d8fc0654836509b419ad241161cc40dff62025a81d";
    let mut privkey = ECDSAPrivateKey::from_string(privkey_str).unwrap();
    println!("{}", privkey.to_string());

    let pk_vec = vec![
        0x3e, 0x3a, 0x84, 0xd1, 0x85, 0xa1, 0x1b, 0xe1, 0xda, 0xaf, 0xad, 0x1d, 0x01, 0xa7, 0xe1,
        0x5e, 0x04, 0x04, 0xab, 0x24, 0xed, 0x4b, 0x8d, 0xe5, 0x89, 0x71, 0xad, 0x93, 0x3e, 0x3f,
        0xc2, 0x4e,
    ];
    privkey = ECDSAPrivateKey::from_vec8(&pk_vec).unwrap();
    println!("{}", privkey.to_string());

    privkey_str = "7009677dc021462d3db7ebc60077b6077f2b15837bf92b46ec5aa45afb820dbc";
    privkey = ECDSAPrivateKey::from_string(privkey_str).unwrap();
    println!("{}", privkey.to_string());

    let msg = hexstr_to_vec("5fff1dae8dc8e2fc4d5b23b2c7665c97f9e9d8edf2b6485a86ba311c25639191b68878628428cccc90b0000000000100a6823403ea3055000000572d3ccdcd0150b64a9339aca0f100000000a8ed32322950b64a9339aca0f1102a8d6aaba430bde80300000000000004454f5300000000083130303130313033000000000000000000000000000000000000000000000000000000000000000000").unwrap();
    let msg_hash = hash_keccak256(&msg);
    println!("msg_hash: {}", vec8_to_hexstr(&msg_hash));
    // let msg_hash =
    //     hexstr_to_vec("7d266152744bf8df4f7a2573d12856a635365fae4e74e19407fe3025a27a7733")?;

    let message = "Hello world";
    let message_u8 = hexstr_to_vec(&hex::encode(message)).unwrap();
    let msg_hash = hash_sha256(&message_u8);
    println!("msg_hash: {}", vec8_to_hexstr(&msg_hash));
    let msg_hash = hash_keccak256(&message_u8);
    println!("msg_hash: {}", vec8_to_hexstr(&msg_hash));

    let sig_info = sign(message, privkey).unwrap();
    println!("signature: {}", vec8_to_hexstr(&sig_info.signature));
    println!("pubkey: {}", vec8_to_hexstr(&sig_info.publickey));

    sig_info.signature
}
