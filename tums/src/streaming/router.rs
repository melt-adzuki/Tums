use anyhow::{Context, Result};
use regex::Regex;

use crate::{
    entities::{ChannelBody, ChannelType, NoteBody, StreamingBody},
    init::SERVICE,
};

pub(crate) async fn route(streaming_body: StreamingBody) -> Result<()> {
    let channel_body: ChannelBody = streaming_body.body;
    let channel_type: ChannelType = channel_body.channel_type;
    let note_body: NoteBody = channel_body.body;
    let text = note_body.text.unwrap_or_default();
    let reply_id = note_body.id;
    let user = note_body.user;

    if user.is_bot {
        return Ok(());
    }

    match channel_type {
        ChannelType::Note if note_body.renote_id.is_none() && !user.is_cat => {
            SERVICE.add_uni_from_dust(text, reply_id).await?;
        }
        ChannelType::Mention => {
            let context = text
                .split_whitespace()
                .filter(|s| {
                    Regex::new(r"^@(?:[a-zA-Z]|\d|_)+(?:@([a-zA-Z]|\d|-)+\.[a-z]+)?$")
                        .is_ok_and(|r| !r.is_match(s))
                })
                .collect::<Vec<_>>();

            let command = *context
                .first()
                .context("No command found on the context.")?;

            match command {
                "show" => SERVICE.show_uni().await?,
                "list" => SERVICE.list_uni(reply_id).await?,
                "add" => {
                    if user.is_cat {
                        SERVICE.cat(reply_id).await?;
                    } else {
                        let pos: i32 = context
                            .get(1)
                            .context("No position command found.")?
                            .parse()?;
                        let content = context
                            .get(2..)
                            .context("No Uni found on the command.")?
                            .join(" ");

                        SERVICE.add_uni(content, pos, reply_id).await?;
                    }
                }
                "remove" => {
                    let pos: i32 = context
                        .get(1)
                        .context("No position command found.")?
                        .parse()?;

                    SERVICE.remove_uni(pos, reply_id).await?;
                }
                _ => {}
            };
        }
        _ => {}
    };

    Ok(())
}
