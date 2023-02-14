use log::*;

use super::body::NoteBody;

pub(crate) fn route(note_body: NoteBody) {
    info!("isRenote: {}", note_body.renote_id.is_some());
    info!(
        "Content : {}\n",
        note_body.text.unwrap_or("None".to_string())
    );
}
