use std::collections::HashMap;

use twilight_model::{
    application::interaction::application_command::CommandOptionValue,
    channel::message::AllowedMentions,
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
    id::{marker::UserMarker, Id},
};
use worker::{kv::KvStore, Response};

use crate::handle::error;

pub async fn tag(
    kv: KvStore,
    options: HashMap<String, CommandOptionValue>,
) -> worker::Result<Response> {
    let name = if let Some(val) = options.get("name") {
        if let CommandOptionValue::String(s) = val {
            s
        } else {
            return error("Discord sent the wrong type for the tag name field.");
        }
    } else {
        return error("Discord failed to send the tag name field.");
    };
    let is_mention = if let Some(CommandOptionValue::User(u)) = options.get("mention") {
        Some(*u)
    } else {
        None
    };
    if let Ok(val) = kv
        .get(name)
        .text_with_metadata::<crate::TagMetadata>()
        .await
    {
        if let Some(content) = val.0 {
            if let Some(metadata) = val.1 {
                if metadata.allow_pings {
                    if let Some(mention) = is_mention {
                        send_tag(format!("<@{}>:\n\n{}", content, mention), Some(mention))
                    } else {
                        send_tag(content, None)
                    }
                } else {
                    send_tag(content, None)
                }
            } else {
                send_tag(content, None)
            }
        } else {
            error(format!("The tag {name} has no content!"))
        }
    } else {
        error(format!("CloudFlare KV error fetching tag {name}"))
    }
}

fn send_tag(
    message: impl ToString,
    maybe_mention: Option<Id<UserMarker>>,
) -> worker::Result<Response> {
    let allowed_mentions = if let Some(mentions) = maybe_mention {
        AllowedMentions::builder().user_ids(vec![mentions]).build()
    } else {
        AllowedMentions::builder().build()
    };
    let resp = InteractionResponseData {
        content: Some(message.to_string()),
        allowed_mentions: Some(allowed_mentions),
        ..Default::default()
    };
    Response::from_json(&InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(resp),
    })
}
