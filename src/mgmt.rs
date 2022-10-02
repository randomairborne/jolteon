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
    invoker: PartialMember,
    guild_id: Id<GuildMarker>,
) -> worker::Result<Response> {
    if let Some(perms) = invoker.permissions {
        if !perms.contains(Permissions::MANAGE_MESSAGES) {
            return error("You need MANAGE_MESSAGES to run this command.")
        }
    } else {
        return error("You need MANAGE_MESSAGES to run this command.")
    }
    options.get()
    Response::empty()
}
