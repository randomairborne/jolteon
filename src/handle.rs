use serde_json::json;
use twilight_model::{
    application::interaction::{Interaction, InteractionType},
    channel::message::MessageFlags,
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
};
use worker::{Request, Response};

pub async fn handle(req: Request, env: worker::Env, data: Interaction) -> worker::Result<Response> {
    if let Ok(kv) = env.kv("tags") {
        match data.kind {
            InteractionType::Ping => Response::from_json(&json!({ "kind": 1 })),
            InteractionType::ApplicationCommand => handle_command(req, kv, data).await,
            InteractionType::ApplicationCommandAutocomplete => {
                handle_autocomplete(req, kv, data).await
            }
            _ => Response::empty(),
        }
    } else {
        error_response("The bot is misconfigured with Cloudflare KV.")
    }
}

async fn handle_command(
    req: Request,
    kv: worker::kv::KvStore,
    data: Interaction,
) -> worker::Result<Response> {
    Response::empty()
}

async fn handle_autocomplete(
    req: Request,
    kv: worker::kv::KvStore,
    data: Interaction,
) -> worker::Result<Response> {
    Response::empty()
}

fn error_response(message: impl ToString) -> worker::Result<Response> {
    let mut flags = MessageFlags::empty();
    flags.insert(MessageFlags::EPHEMERAL);
    let resp = InteractionResponseData {
        flags: Some(flags),
        content: Some(message.to_string()),
        ..Default::default()
    };
    Response::from_json(&InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(resp),
    })
}
