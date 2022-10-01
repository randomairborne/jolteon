use std::collections::HashMap;

use twilight_model::application::interaction::application_command::CommandOptionValue;
use worker::{Response, kv::KvStore};

use crate::handle::error;

pub async fn manage(
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
    let mention = if let Some(CommandOptionValue::User(u)) = options.get("mention") {
        Some(*u)
    } else {
        None
    };
    Response::empty()
}