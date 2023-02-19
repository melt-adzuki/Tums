use anyhow::{Context, Result};
use regex::Regex;

use crate::init::SERVICE;

use super::body::{ChannelBody, ChannelType, NoteBody, StreamingBody};

pub(crate) async fn route(streaming_body: StreamingBody) -> Result<()> {
    let channel_body: ChannelBody = streaming_body.body;
    let channel_type: ChannelType = channel_body.channel_type;
    let note_body: NoteBody = channel_body.body;

    match channel_type {
        ChannelType::Note if note_body.renote_id.is_none() => {
            let content = note_body.text.unwrap_or_default();
            SERVICE.add_uni_from_dust(content, note_body.id).await?;
        }
        ChannelType::Mention | ChannelType::Reply => {
            let context = note_body.text.unwrap_or_default();
            let context = context
                .split_whitespace()
                .filter(|s| {
                    Regex::new(r"^@(?:[a-zA-Z]|\d|_)+(?:@([a-zA-Z]|\d|-)+\.[a-z]+)?$")
                        .is_ok_and(|r| !r.is_match(s))
                })
                .collect::<Vec<_>>();

            let command = *context
                .first()
                .context("The command of the context cannot be found.")?;

            match command {
                "show" => SERVICE.show_uni().await?,
                "list" => SERVICE.list_uni().await?,
                "add" => {
                    let pos: i32 = context
                        .get(1)
                        .context("The position command cannot be found.")?
                        .parse()?;
                    let content = context
                        .get(2..)
                        .context("The context on the command cannot be found.")?
                        .join(" ");
                    SERVICE.add_uni(content, pos).await?;
                }
                "remove" => {
                    let pos: i32 = context
                        .get(1)
                        .context("The position command cannot be found.")?
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
