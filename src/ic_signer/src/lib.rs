mod crypto;
mod types;
mod utils;

use types::{Bundle, ECDSAPrivateKey, HashAlgorithm, PrivateKey};
use utils::{hash_keccak256, hash_sha256, hexstr_to_vec, vec8_to_hexstr, verify_signature};
// use k256::sha2::{Sha256, Sha512, Digest};

// use ic_cdk::api::call::CallResult;
use ic_cdk::{
    api,
    export::{
        candid::{CandidType, Deserialize, Func, Nat},
        serde::{Deserialize as SerdeDeserialize, Serialize},
        Principal,
    },
};
use serde_json;
use std::{cell::RefCell, collections::BTreeMap, mem};

#[derive(Clone, CandidType, Deserialize)]
struct HttpHeader(String, String);

#[derive(Debug, Clone, CandidType, Deserialize)]
struct Token {
    key: String,
    content_encoding: String,
    index: Nat,
    sha256: Option<u8>,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
struct CallbackStrategy {
    /// The callback function to be called to continue the stream.
    callback: Func,
    /// The token to pass to the function.
    token: Token,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
enum StreamingStrategy {
    /// A callback-based streaming strategy, where a callback function is provided for continuing the stream.
    Callback(CallbackStrategy),
}

#[derive(CandidType, Deserialize)]
struct HttpResponse {
    status_code: u16,
    body: Vec<u8>,
    headers: Vec<HttpHeader>,
    streaming_strategy: Option<StreamingStrategy>,
    upgrade: Option<bool>,
}

#[derive(CandidType, Deserialize)]
struct HttpRequest {
    url: String,
    method: String,
    body: Option<Vec<u8>>,
    headers: Vec<HttpHeader>,
}

#[derive(serde::Deserialize)]
struct JsonRPC {
    id: String,
    method: String,
    params: Vec<String>,
}

// curl http://localhost:8000/?canisterId=rrkah-fqaaa-aaaaa-aaaaq-cai
#[ic_cdk_macros::query]
fn http_request(request: HttpRequest) -> HttpResponse {
    let mut status_code = 404;
    let mut res_body = String::new();
    let mut id = String::new();
    let mut result = String::new();

    if request.method.to_ascii_lowercase() == "post" {
        status_code = 400;

        let parse_res = &parse_request(&request);
        match parse_res {
            Ok(res) => {
                let method = &res.method;
                let params = &res.params;

                let privkey_str = &params[0];
                let digest = &params[1];
                if method.to_ascii_lowercase() == "sign_digest" {
                    let sig_info = sign_digest(digest, privkey_str).unwrap();
                    result = vec8_to_hexstr(&sig_info.signature);
                    id = res.id.to_string();
                    status_code = 200;
                }
            }
            Err(e) => result = e.to_string(),
        }
    }

    res_body = format!(
        "{{\"code\":{}, \"id\":\"{}\", \"result\":\"{}\"}}\n",
        status_code, id, result
    );
    let headers = [
        HttpHeader(
            "content-type".to_string(),
            "application/json; charset=utf-8".to_string(),
        ),
        HttpHeader(
            "content-length".to_string(),
            res_body.as_bytes().len().to_string(),
        ),
    ];
    HttpResponse {
        body: res_body.as_bytes().to_vec(),
        headers: headers.to_vec(),
        status_code,
        streaming_strategy: None,
        upgrade: Some(false),
    }
}

fn parse_request(request: &HttpRequest) -> Result<JsonRPC, String> {
    match request.body.clone() {
        Some(body) => {
            let body_str = String::from_utf8(body).unwrap();

            let json_parsed = serde_json::from_str(&body_str);
            match json_parsed {
                Ok(res) => {
                    return Ok(res);
                }
                Err(_) => return Err("Failed to parse request body".to_string()),
            };
        }
        _ => return Err("Empty request body".to_string()),
    }
}

#[derive(CandidType, Deserialize, Default)]
struct State {
    privkeys: BTreeMap<Principal, BTreeMap<String, String>>,
}

thread_local! {
    // static RSA_KEYS: RefCell<BTreeMap<& 'static u64, >>
    static STATE: RefCell<State> = RefCell::default();
}

impl State {
    pub fn get_privkey(principal: &Principal, key_id: &str) -> Result<String, String> {
        let state = STATE.with(|s| mem::take(&mut *s.borrow_mut()));
        match state.privkeys.get(principal) {
            Some(pk_map) => match pk_map.get(key_id) {
                Some(pk) => Ok(pk.clone()),
                None => Err("Key ID not found".to_string()),
            },
            None => Err("Principal not found".to_string()),
        }
        // "6a73b985cfd0142ba4be36d8fc0654836509b419ad241161cc40dff62025a81d".to_string()
    }

    pub fn set_privkey(principal: &Principal, key_id: &str, key: &str) -> Result<(), String> {
        let state = STATE.with(|s| mem::take(&mut *s.borrow_mut()));
        let mut privkeys = state.privkeys;
        match privkeys.get(principal) {
            Some(pk_map) => match pk_map.get(key_id) {
                Some(_) => return Err("This key ID is already exist".to_string()),
                None => {
                    let mut new_pk_map = pk_map.clone();
                    new_pk_map.insert(String::from(key_id), String::from(key));
                    privkeys.insert(*principal, new_pk_map);
                }
            },
            None => {
                let mut new_pk_map = BTreeMap::new();
                new_pk_map.insert(String::from(key_id), String::from(key));
                privkeys.insert(*principal, new_pk_map);
            }
        };
        STATE.with(|s| *s.borrow_mut() = State { privkeys });
        Ok(())
    }
}

#[ic_cdk_macros::update]
fn upload_privkey(key_id: String, key: String) -> String {
    let caller = api::caller();
    State::set_privkey(&caller, &key_id, &key).unwrap();
    vec8_to_hexstr(&caller.as_ref().to_vec())
}

#[ic_cdk_macros::query]
fn show_privkey(key_id: String) -> String {
    let caller = api::caller();
    let key = State::get_privkey(&caller, &key_id).unwrap();
    key
}

#[ic_cdk_macros::query]
fn sign_digest_mpc(digest: String, key_id: String) -> String {
    let res = sign_digest(&digest, &key_id);
    match res {
        Ok(res) => serde_json::to_string(&res).unwrap(),
        Err(err) => format!("{{\"result\":\"{}\"}}\n", err).to_string(),
    }
}

fn sign_digest(digest: &str, private_key: &str) -> Result<Bundle, String> {
    let privkey = ECDSAPrivateKey::from_string(private_key)?;
    let hash_algo = HashAlgorithm::Keccak256;

    let msg_hash = hexstr_to_vec(digest)?;

    let sig = privkey.sign(&msg_hash, hash_algo)?;
    let pubkey = privkey.to_pubkey()?;

    let verified = verify_signature(&msg_hash, &sig, &pubkey);
    if verified {
        Ok(Bundle {
            digest: msg_hash,
            publickey: pubkey,
            signature: sig,
        })
    } else {
        return Err("Signature verified failed".to_string());
    }
}

type CanisterId = Principal;

#[derive(CandidType, Serialize, Debug)]
struct ECDSAPublicKey {
    pub canister_id: Option<CanisterId>,
    pub derivation_path: Vec<Vec<u8>>,
    pub key_id: EcdsaKeyId,
}

#[derive(CandidType, SerdeDeserialize, Debug)]
struct ECDSAPublicKeyReply {
    pub public_key: Vec<u8>,
    pub chain_code: Vec<u8>,
}

#[derive(CandidType, Serialize, Debug)]
struct SignWithECDSA {
    pub message_hash: Vec<u8>,
    pub derivation_path: Vec<Vec<u8>>,
    pub key_id: EcdsaKeyId,
}

#[derive(CandidType, SerdeDeserialize, Debug)]
struct SignWithECDSAReply {
    pub signature: Vec<u8>,
}

#[derive(CandidType, Serialize, Debug, Clone)]
struct EcdsaKeyId {
    pub curve: EcdsaCurve,
    pub name: String,
}

#[derive(CandidType, Serialize, Debug, Clone)]
pub enum EcdsaCurve {
    #[serde(rename = "secp256k1")]
    Secp256k1,
}

#[ic_cdk_macros::update]
async fn sign_digest_ic(digest: String) -> String {
    let msg_hash = match hexstr_to_vec(&digest) {
        Ok(hash) => hash,
        Err(_) => {
            return format!("{{\"result\":\"Sign failed when decoding digest\"}}\n").to_string()
        }
    };
    assert!(msg_hash.len() == 32);

    let ic00 = ic_cdk::export::Principal::management_canister();
    // let (rnd_buf,): (Vec<u8>,) = match ic_cdk::call(ic00, "raw_rand", ()).await {
    //     Ok(res) => res,
    //     Err(_) => return "Sign failed when parsing request body".to_string(),
    // };
    // return vec8_to_hexstr(&rnd_buf);
    // let ic00_canister_id = "aaaaa-aa".to_string();
    let key_id = EcdsaKeyId {
        curve: EcdsaCurve::Secp256k1,
        name: "".to_string(),
    };
    // let this_canister_id = "ptf43-biaaa-aaaai-ack3q-cai".to_string();
    // let this_canister = CanisterId::from_str(&this_canister_id).unwrap();
    // let pubkey: Vec<u8> = {
    //     let request = ECDSAPublicKey {
    //         canister_id: None, // Some(this_canister),
    //         derivation_path: vec![vec![2, 3]],
    //         key_id: key_id.clone(),
    //     };
    //     // ic_cdk::println!("Sending signature request = {:?}", request);
    //     let (res,): (ECDSAPublicKeyReply,) = ic_cdk::call(ic00, "ecdsa_public_key", (request,))
    //         .await
    //         .map_err(|e| format!("Failed to call ecdsa_public_key: {}", e.1))
    //         .unwrap();
    //     // ic_cdk::println!("Got response = {:?}", res);
    //     res.public_key
    // };

    let sig: Vec<u8> = {
        let request = SignWithECDSA {
            message_hash: msg_hash.clone(),
            derivation_path: vec![vec![2, 3]],
            key_id,
        };
        // ic_cdk::println!("Sending signature request = {:?}", request);
        let (res,): (SignWithECDSAReply,) = ic_cdk::call(ic00, "sign_with_ecdsa", (request,))
            .await
            .map_err(|e| format!("Failed to call sign_with_ecdsa {}", e.1))
            .unwrap();
        // ic_cdk::println!("Got response = {:?}", res);
        res.signature
    };

    let verified = true; // verify_signature(&msg_hash, &sig, &pubkey);
    if verified {
        let res = Bundle {
            digest: msg_hash,
            publickey: Vec::new(), // pubkey,
            signature: sig,
        };
        return serde_json::to_string(&res).unwrap();
    } else {
        return format!("{{\"result\":\"Sign failed, signature verified failed\"}}\n").to_string();
    }
}

#[test]
fn test_parse_request() {
    let body = Some(
        r#"{
        "id":"15",
        "method":"sign_digest",
        "params":[
            "6a73b985cfd0142ba4be36d8fc0654836509b419ad241161cc40dff62025a81d",
            "369183d3786773cef4e56c7b849e7ef5f742867510b676d6b38f8e38a222d8a2"
        ]
    }"#
        .as_bytes()
        .to_vec(),
    );
    let req = HttpRequest {
        url: "/?canisterId=rrkah-fqaaa-aaaaa-aaaaq-cai".to_string(),
        method: "POST".to_string(),
        body: body,
        headers: vec![],
    };

    let res = parse_request(&req).unwrap();
    println!("id: {}", res.id);
}

// dfx canister --network ic --wallet "$(dfx identity --network ic get-wallet)" update-settings --all --add-controller "$(dfx identity get-principal)"
#[test]
fn test_sign() {
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

    let message = "Hello world"; // 369183d3786773cef4e56c7b849e7ef5f742867510b676d6b38f8e38a222d8a2
    let message_u8 = hexstr_to_vec(&hex::encode(message)).unwrap();
    let msg_hash = hash_sha256(&message_u8);
    println!("msg_hash: {}", vec8_to_hexstr(&msg_hash));
    let msg_hash = hash_keccak256(&message_u8);
    println!("msg_hash: {}", vec8_to_hexstr(&msg_hash));

    let sig_info = sign_digest(message, &privkey.to_string()).unwrap();
    println!("signature: {}", vec8_to_hexstr(&sig_info.signature));
    println!("pubkey: {}", vec8_to_hexstr(&sig_info.publickey));
}
