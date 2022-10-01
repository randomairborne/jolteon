use ed25519_dalek::{PublicKey, Signature, Verifier};
use twilight_model::application::interaction::Interaction;
use worker::*;

mod tag;
mod handle;
mod mgmt;

#[allow(dead_code)]
#[event(fetch)]
async fn main(mut req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    console_error_panic_hook::set_once();
    let body = req.bytes().await.unwrap();
    validate_discord_sig(
        req.headers(),
        &body,
        env.secret("DISCORD_PUBLIC_KEY").unwrap().to_string(),
    )
    .unwrap();
    let item: Interaction = serde_json::from_slice(&body).unwrap();
    handle::handle(env, item).await
}

// TODO clean this up and use less unwraps
fn validate_discord_sig(
    headers: &Headers,
    body: &[u8],
    pub_key_string: String,
) -> std::result::Result<(), ed25519_dalek::SignatureError> {
    let sig_hex = hex::decode(headers.get("X-Signature-Ed25519").unwrap().unwrap()).unwrap();
    let mut sig_arr: [u8; 64] = [0; 64];
    for (i, byte) in sig_hex.into_iter().enumerate() {
        sig_arr[i] = byte;
    }
    let sig = Signature::from_bytes(&sig_arr)?;
    let timestamp = headers.get("X-Signature-Timestamp").unwrap().unwrap();
    let pub_key = PublicKey::from_bytes(&hex::decode(pub_key_string).unwrap()).unwrap();
    let to_be_verified: Vec<u8> = timestamp
        .as_bytes()
        .iter()
        .chain(body.iter())
        .cloned()
        .collect();
    pub_key.verify(to_be_verified.as_slice(), &sig)?;
    Ok(())
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy)]
pub struct TagMetadata {
    pub allow_pings: bool,
}