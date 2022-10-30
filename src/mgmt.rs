use std::collections::HashMap;

use twilight_model::{
    application::interaction::application_command::CommandOptionValue,
    guild::{PartialMember, Permissions},
    http::interaction::{InteractionResponse, InteractionResponseType},
    id::{marker::GuildMarker, Id},
};
use twilight_util::builder::InteractionResponseDataBuilder;
use worker::{kv::KvStore, Response};

use crate::handle::error;

pub async fn manage(
    kv: KvStore,
    options: HashMap<String, CommandOptionValue>,
    guild_id: Id<GuildMarker>,
) -> InteractionResponse {
    let ird = InteractionResponseDataBuilder::new().content(msg).build();
    InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(ird),
    }
}
