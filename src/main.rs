use std::fs;
use serde::{Deserialize, Serialize};
use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
use base64::{engine::general_purpose, Engine as _};

#[derive(Deserialize)]
struct MsgInput {
    msg: String,
}

#[derive(Serialize)]
struct Output {
    message: String,
    signature: String,
    public_key: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //
    // 1. Load msg.json
    //
    let msg_json = fs::read_to_string("msg.json")?;
    let msg_input: MsgInput = serde_json::from_str(&msg_json)?;
    let message = msg_input.msg;

    //
    // 2. Load Base64 private key (32 bytes)
    //
    let priv_b64 = fs::read_to_string("my_ed25519_priv_key_for_sign")?;
    let priv_raw = general_purpose::STANDARD.decode(priv_b64.trim())?;

    if priv_raw.len() != 32 {
        panic!("Private key must be 32 bytes Ed25519 seed.");
    }

    //
    // 3. Construct SigningKey
    //
    let signing_key = SigningKey::from_bytes(&priv_raw.try_into().unwrap());

    //
    // 4. Get public key
    //
    let verifying_key: VerifyingKey = signing_key.verifying_key();
    let pub_b64 = general_purpose::STANDARD.encode(verifying_key.to_bytes());

    //
    // 5. Sign message
    //
    let signature = signing_key.sign(message.as_bytes());
    let sig_b64 = general_purpose::STANDARD.encode(signature.to_bytes());

    //
    // 6. Output JSON
    //
    let output = Output {
        message,
        signature: sig_b64,
        public_key: pub_b64,
    };

    let json_text = serde_json::to_string_pretty(&output)?;
    fs::write("alice.json", json_text)?;

    println!("alice.json generated successfully!");

    Ok(())
}