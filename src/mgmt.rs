use std::collections::HashMap;

use twilight_model::{
    application::interaction::application_command::CommandOptionValue,
    guild::{PartialMember, Permissions},
    id::{marker::GuildMarker, Id},
};
use worker::{kv::KvStore, Response};

use crate::handle::error;

pub async fn manage(
    kv: KvStore,
    options: HashMap<String, CommandOptionValue>,
    guild_id: Id<GuildMarker>,
) -> worker::Result<Response> {
}
