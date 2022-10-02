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
    let name = if let Some(val) = options.get("name") {
        if let CommandOptionValue::String(s) = val {
            s
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
    Response::empty()
}
