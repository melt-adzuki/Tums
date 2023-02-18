use anyhow::Result;

use crate::init::SERVICE;

use super::body::NoteBody;

pub(crate) async fn route(note_body: NoteBody) -> Result<()> {
    let is_renote = note_body.renote_id.is_some();
    if is_renote {
        return Ok(());
    };

    let content = note_body.text.unwrap_or_default();

    SERVICE.add_uni_from_dust(content, note_body.id).await?;
    Ok(())
}
