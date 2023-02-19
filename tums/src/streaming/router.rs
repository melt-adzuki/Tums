use anyhow::{Context, Result};
use regex::Regex;

use crate::init::SERVICE;

use super::body::{ChannelBody, ChannelType, NoteBody, StreamingBody};

pub(crate) async fn route(streaming_body: StreamingBody) -> Result<()> {
    let channel_body: ChannelBody = streaming_body.body;
    let channel_type: ChannelType = channel_body.channel_type;
    let note_body: NoteBody = channel_body.body;
    let text = note_body.text.unwrap_or_default();

    match channel_type {
        ChannelType::Note if note_body.renote_id.is_none() => {
            SERVICE.add_uni_from_dust(text, note_body.id).await?
        }
        ChannelType::Mention | ChannelType::Reply => {
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
                "list" => SERVICE.list_uni().await?,
                "add" => {
                    let pos: i32 = context
                        .get(1)
                        .context("No position command found.")?
                        .parse()?;
                    let content = context
                        .get(2..)
                        .context("No Uni found on the command.")?
                        .join(" ");
                    SERVICE.add_uni(content, pos).await?;
                }
                "remove" => {
                    let pos: i32 = context
                        .get(1)
                        .context("No position command found.")?
                        .parse()?;
                    let reply_id = note_body.id;
                    SERVICE.remove_uni(pos, reply_id).await?;
                }
                _ => {}
            };
        }
        _ => {}
    };

    Ok(())
}
