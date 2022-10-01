use twilight_model::{
    channel::message::AllowedMentions,
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType}, application::interaction::application_command::CommandData,
};
use worker::{kv::KvStore, Response};

pub async fn tag(kv: KvStore, cmd: CommandData) -> worker::Result<Response> {
    cmd.
}

fn send_tag(message: impl ToString, mentions: Option<>) -> worker::Result<Response> {
    let allowed_mentions_builder = AllowedMentions::builder().user_ids();
    let allowed_mentions = allowed_mentions_builder.users().build();
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
