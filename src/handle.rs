use std::collections::HashMap;

use twilight_model::{
    application::interaction::{
        application_command::{CommandData, CommandOptionValue},
        Interaction, InteractionData,
    },
    channel::message::{AllowedMentions, MessageFlags},
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
};
use worker::Response;
pub async fn handle(env: worker::Env, data: Interaction) -> worker::Result<Response> {
    let kv = env.kv("tags")?;
    if let Some(data) = data.data {
        if let InteractionData::ApplicationCommand(cmd) = data {
            handle_command(kv, *cmd).await
        } else {
            error("InteractionData was wrong type")
        }
    } else {
        error("Interaction did not contain InteractionData!")
    }
}

async fn handle_command(kv: worker::kv::KvStore, cmd: CommandData) -> worker::Result<Response> {
    let options: HashMap<String, CommandOptionValue> = cmd
        .options
        .iter()
        .map(|t| (t.name.clone(), t.value.clone()))
        .collect();
    match cmd.name.as_str() {
        "tag" => crate::tag::tag(kv, options).await,
        "tagmanage" => crate::mgmt::manage(kv, options).await,
        _ => error("That command is not recognized"),
    }
}

pub fn error(message: impl ToString) -> worker::Result<Response> {
    let mut flags = MessageFlags::empty();
    flags.insert(MessageFlags::EPHEMERAL);
    let allowed_mentions = AllowedMentions::builder().build();
    let resp = InteractionResponseData {
        flags: Some(flags),
        content: Some(message.to_string()),
        allowed_mentions: Some(allowed_mentions),
        ..Default::default()
    };
    Response::from_json(&InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(resp),
    })
}
