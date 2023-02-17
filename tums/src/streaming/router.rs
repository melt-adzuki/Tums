use anyhow::Result;

use crate::init::{SERVICE, TOKIO_RUNTIME};

use super::body::NoteBody;

pub(crate) fn route(note_body: NoteBody) -> Result<()> {
    let is_renote = note_body.renote_id.is_some();
    if is_renote {
        return Ok(());
    };

    let content = note_body.text.unwrap_or_default();

    TOKIO_RUNTIME.block_on(async {
        SERVICE.add_uni_from_dust(content, note_body.id).await?;
        Ok(())
    })
}
