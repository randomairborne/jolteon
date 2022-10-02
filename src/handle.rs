use std::collections::HashMap;

use twilight_model::{
    application::interaction::{
        application_command::CommandOptionValue, Interaction, InteractionData,
    },
    channel::message::{AllowedMentions, MessageFlags},
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
};
use worker::Response;
pub async fn handle(env: worker::Env, data: Interaction) -> worker::Result<Response> {
    if !data.is_guild() || data.guild_id.is_none() {
        return error(
            "Jolteon only works in guilds. Invite it with <https://valk.sh/jolteon-invite>!",
        );
    }
    let invoker = if let Some(member) = data.member {
        member
    } else {
        return error("No member sent with tag!");
    };
    let guild_id = if let Some(guild_id) = data.guild_id {
        guild_id
    } else {
        return error("No guild ID - This should be unreachable!");
    };
    let kv = env.kv("tags")?;
    if let Some(data) = data.data {
        if let InteractionData::ApplicationCommand(cmd) = data {
            let options: HashMap<String, CommandOptionValue> = cmd
                .options
                .iter()
                .map(|t| (t.name.clone(), t.value.clone()))
                .collect();
            match cmd.name.as_str() {
                "tag" => crate::tag::tag(kv, options, guild_id).await,
                "tagmanage" => crate::mgmt::manage(kv, options, invoker, guild_id).await,
                _ => error("That command is not recognized."),
            }
        } else {
            error("InteractionData was wrong type")
        }
    } else {
        error("Interaction did not contain InteractionData!")
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
