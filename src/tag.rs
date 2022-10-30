use std::collections::HashMap;

use twilight_model::{
    application::{interaction::application_command::CommandOptionValue, command::CommandOptionChoice},
    channel::message::AllowedMentions,
    http::interaction::{InteractionResponse, InteractionResponseType},
    id::{
        marker::{GuildMarker, UserMarker},
        Id,
    },
};
use twilight_util::builder::InteractionResponseDataBuilder;
use worker::kv::KvStore;

use crate::handle::error;

pub async fn tag(
    kv: KvStore,
    options: HashMap<String, CommandOptionValue>,
    guild_id: Id<GuildMarker>,
) -> InteractionResponse {
    let name = if let Some(val) = options.get("name") {
        if let CommandOptionValue::String(s) = val {
            s.trim()
        } else {
            return error("Discord sent the wrong type for the tag name field.");
        }
    } else {
        return error("Discord failed to send the tag name field.");
    };
    let mention = if let Some(CommandOptionValue::User(u)) = options.get("mention") {
        Some(*u)
    } else {
        None
    };
    if let Ok(val) = kv
        .get(&format!("{guild_id}-{name}"))
        .text_with_metadata::<crate::TagMetadata>()
        .await
    {
        if let Some(content) = val.0 {
            if let Some(metadata) = val.1 {
                send_tag(content, mention, metadata.allow_pings)
            } else {
                send_tag(content, mention, false)
            }
        } else {
            error(format!("The tag {name} has no content!"))
        }
    } else {
        error(format!("CloudFlare KV error fetching tag {name}"))
    }
}

pub async fn autocomplete(kv: KvStore, guild_id: Id<GuildMarker>,) -> InteractionResponse {
    let kv_options = if let Ok(val) = kv
        .list().prefix(&format!("{guild_id}-{name}")).execute()
        .await
    {
        val
    } else {
        error(format!("CloudFlare KV error fetching tag {name}"))
    }
    CommandOptionChoice
    let ird = InteractionResponseDataBuilder::new().choices(choices);
    InteractionResponse {
        kind: InteractionResponseType::ApplicationCommandAutocompleteResult,
        data: Some(ird)
    }
}

fn send_tag(
    message: impl ToString,
    wants_to_mention: Option<Id<UserMarker>>,
    can_mention: bool,
) -> InteractionResponse {
    let mut message = message.to_string();
    let allowed_mentions = if let Some(mentions) = wants_to_mention {
        if can_mention {
            AllowedMentions::builder().user_ids(vec![mentions]).build()
        } else {
            AllowedMentions::builder().build()
        }
    } else {
        AllowedMentions::builder().build()
    };
    if let Some(mention) = wants_to_mention {
        message = format!("<@!{}>:\n\n{}", mention, message);
    }
    let resp = InteractionResponseDataBuilder::new()
        .content(message)
        .allowed_mentions(allowed_mentions)
        .build();
    InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(resp),
    }
}
