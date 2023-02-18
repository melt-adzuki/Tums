use anyhow::Result;

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
        ChannelType::Mention => {
            todo!()
        }
        ChannelType::Reply => {
            todo!()
        }
        _ => {}
    };

    Ok(())
}
