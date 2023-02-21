use anyhow::Result;

use crate::{
    domain::interactor::Interactor,
    entities::{ChannelBody, StreamingBody, User},
    init::SERVICE,
    log,
};

use super::commands::command;

pub(crate) async fn route(me: &User, streaming_body: &StreamingBody) -> Result<()> {
    let channel_body = &streaming_body.body;

    match channel_body {
        ChannelBody::Note { body }
            if body.renote_id.is_none()
                && !body.user.is_cat
                && body.user.id != me.id
                && !body.user.is_bot =>
        {
            log!(
                "INFO" | "From {} <<< {}: {}",
                body.user.id.green().bold(),
                "A note recieved".bright_blue(),
                body.id
            );

            let text = body.text.clone().unwrap_or_default();
            let reply_id = body.id.to_string();

            SERVICE
                .add_uni_from_dust(text, reply_id.to_string())
                .await?;
        }
        ChannelBody::Mention { body } if body.user.id != me.id && !body.user.is_bot => {
            log!(
                "INFO" | "From {} <<< {}: {}",
                body.user.id.green().bold(),
                "A mention recieved".bright_blue(),
                body.id
            );

            let text = body.text.clone().unwrap_or_default();
            let reply_id = &body.id;

            match command(&text, &body.user, reply_id.to_string()).await {
                Ok(_) => {}
                Err(err) => {
                    SERVICE
                        .interactor
                        .reply(err.to_string(), reply_id.to_string())
                        .await?;
                }
            }
        }
        ChannelBody::Followed { body } => SERVICE.interactor.subscribe(body.id.to_string()).await?,
        _ => {}
    };

    Ok(())
}
