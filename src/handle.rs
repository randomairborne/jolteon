use serde_json::json;
use twilight_model::application::{
    command::CommandType, interaction::application_command::CommandData,
};
use worker::{Request, Response};

pub async fn handle(req: Request, data: CommandData) -> worker::Result<Response> {
    match data.kind {
        CommandType::Unknown(kind) => Response::from_json(&json!({ "type": kind })),
        CommandType::ChatInput => Response::from_json(),
        _ => Response::empty(),
    }
}
