use std::collections::HashMap;

use twilight_model::{
    application::interaction::{
        application_command::CommandOptionValue, Interaction, InteractionData, InteractionType,
    },
    channel::message::MessageFlags,
    http::interaction::{InteractionResponse, InteractionResponseType},
};
use twilight_util::builder::InteractionResponseDataBuilder;
pub async fn handle(env: worker::Env, interaction: Interaction) -> InteractionResponse {
    if !interaction.is_guild() || interaction.guild_id.is_none() {
        return error(
            "Jolteon only works in guilds. Invite it with <https://valk.sh/jolteon-invite>!",
        );
    }
    let guild_id = if let Some(guild_id) = interaction.guild_id {
        guild_id
    } else {
        return error("No guild ID - This should be unreachable!");
    };
    let kv = match env.kv("tags") {
        Ok(val) => val,
        Err(e) => return error(format!("No KV: {e}")),
    };
    if let Some(data) = interaction.data {
        match interaction.kind {
            InteractionType::Ping => {
            InteractionResponse {
                    kind: InteractionResponseType::Pong,
                    data: None,
                }
            }
            InteractionType::ApplicationCommand => {
                if let InteractionData::ApplicationCommand(cmd) = data {
                    let options: HashMap<String, CommandOptionValue> =
                        cmd.options.into_iter().map(|t| (t.name, t.value)).collect();
                    match cmd.name.as_str() {
                        "tag" => crate::tag::tag(kv, options, guild_id).await,
                        "tagmanage" => crate::mgmt::manage(kv, options, guild_id).await,
                        _ => error("That command is not recognized."),
                    }
                } else {
                    error("InteractionData was wrong type!")
                }
            }
            InteractionType::ApplicationCommandAutocomplete => {if let InteractionData::ApplicationCommand(cmd) = data {
                let options: HashMap<String, CommandOptionValue> =
                    cmd.options.into_iter().map(|t| (t.name, t.value)).collect();
                    if let Some(CommandOptionValue::String(tn)) = options.get("name") {
                    crate::tag::autocomplete(kv, guild_id,tn ).await} else {error("tag name was not sent!")}
            } else {
                error("InteractionData was wrong type!")
            }},
            _ => error("Only pings, ApplicationCommands, and ApplicationCommandAutocompletes are supported."),
        }
    } else {
        error("Interaction did not contain InteractionData!")
    }
}

pub fn error(message: impl ToString) -> InteractionResponse {
    let resp = InteractionResponseDataBuilder::new()
        .content(message.to_string())
        .flags(MessageFlags::EPHEMERAL)
        .build();
    InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(resp),
    }
}
