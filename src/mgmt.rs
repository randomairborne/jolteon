use std::collections::HashMap;

use twilight_model::{
    application::interaction::application_command::CommandOptionValue,
    guild::{PartialMember, Permissions},
    http::interaction::{InteractionResponse, InteractionResponseType},
    id::{marker::GuildMarker, Id},
};
use worker::{kv::KvStore, Response};

use crate::handle::error;

pub async fn manage(
    kv: KvStore,
    options: HashMap<String, CommandOptionValue>,
    guild_id: Id<GuildMarker>,
) -> InteractionResponse {
    InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: None,
    }
}
