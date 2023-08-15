use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use twilight_model::application::interaction::Interaction;
use worker::*;

mod cmds;
mod handle;
mod mgmt;
mod tag;

#[allow(dead_code)]
#[event(fetch)]
async fn main(mut req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    console_error_panic_hook::set_once();
    let body = req.bytes().await.unwrap();
    validate_discord_sig(
        req.headers(),
        &body,
        &env.secret("DISCORD_PUBLIC_KEY").unwrap().to_string(),
    )
    .unwrap();
    let item: Interaction = serde_json::from_slice(&body).unwrap();
    Response::from_json(&handle::handle(env, item).await)
}

pub fn validate_discord_sig(
    headers: &Headers,
    body: &[u8],
    pub_key_string: &str,
) -> std::result::Result<(), SignatureValidationError> {
    let sig_arr = hex::decode(
        headers
            .get("X-Signature-Ed25519")?
            .ok_or(SignatureValidationError::MissingSignatureHeader)?,
    )?;
    let sig = Signature::from_slice(&sig_arr)?;
    let timestamp = headers
        .get("X-Signature-Timestamp")?
        .ok_or(SignatureValidationError::MissingTimestampHeader)?;
    let pub_key =
        VerifyingKey::from_bytes(&hex::decode(pub_key_string)?.try_into().unwrap_or([0; 32]))?;
    let to_be_verified: Vec<u8> = timestamp
        .as_bytes()
        .iter()
        .chain(body.iter())
        .copied()
        .collect();
    pub_key.verify(to_be_verified.as_slice(), &sig)?;
    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum SignatureValidationError {
    #[error("ed25519-dalek signature error")]
    Dalek(#[from] ed25519_dalek::SignatureError),
    #[error("Hex decode error")]
    HexDecode(#[from] hex::FromHexError),
    #[error("Header fetch error")]
    Worker(#[from] Error),
    #[error("Missing X-Signature-Ed25519 header")]
    MissingSignatureHeader,
    #[error("Missing X-Signature-Timestamp header")]
    MissingTimestampHeader,
}

// #[event(scheduled)]
// async fn scheduled(event: worker::ScheduledEvent, env: worker::Env, ctx: worker::ScheduleContext) {}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy)]
pub struct TagMetadata {
    pub allow_pings: bool,
    pub hidden: bool,
}
