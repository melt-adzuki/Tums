use anyhow::*;
use log::info;

use super::body::NoteBody;

pub(crate) trait Note {
    async fn post(&self) -> Result<()>;
}

impl Note for NoteBody {
    async fn post(&self) -> anyhow::Result<()> {
        // いい感じ™の処理
        let body = serde_json::to_string(self)?;
        info!("{}", body);

        todo!()
    }
}
