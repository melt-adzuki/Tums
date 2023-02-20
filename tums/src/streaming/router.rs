use anyhow::Result;

use crate::{
    domain::interactor::Interactor,
    entities::{ChannelType, StreamingBody, User},
    init::SERVICE,
};

use super::commands::command;

pub(crate) async fn route(me: &User, streaming_body: &StreamingBody) -> Result<()> {
    let channel_body = &streaming_body.body;
    let channel_type = &channel_body.channel_type;
    let note_body = &channel_body.body;
    let text = note_body.text.clone().unwrap_or_default();
    let reply_id = &note_body.id;
    let user = &note_body.user;

    if user.id == me.id || user.is_bot {
        return Ok(());
    }

    match channel_type {
        ChannelType::Note if note_body.renote_id.is_none() && !user.is_cat => {
            SERVICE
                .add_uni_from_dust(text, reply_id.to_string())
                .await?;
        }
        ChannelType::Mention => match command(&text, user, reply_id.to_string()).await {
            Ok(_) => {}
            Err(err) => {
                SERVICE
                    .interactor
                    .reply(err.to_string(), reply_id.to_string())
                    .await?;
            }
        },
        _ => {}
    };

    Ok(())
}
