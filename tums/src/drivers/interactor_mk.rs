use ::serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{confs::CONFS, domain::interactor::Interactor, entities::NoteBody, log};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct NoteCreateResponse {
    created_note: NoteBody,
}

pub(crate) struct InteractorMisskeyImpl {
    client: reqwest::Client,
}

impl InteractorMisskeyImpl {
    pub(crate) fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    async fn create_note(
        &self,
        content: String,
        reply_id: Option<String>,
    ) -> anyhow::Result<NoteCreateResponse> {
        log!(
            "INFO" | "To {} >>> {}",
            match reply_id.clone() {
                Some(reply_id) => reply_id.green().bold(),
                None => "Timeline".yellow().bold(),
            },
            "Creating a note...".cyan()
        );

        let mut json = json!({
            "i": CONFS.mk_token,
            "text": content,
            "visibility": "home",
        });

        if let Some(reply_id) = reply_id {
            json["replyId"] = json!(reply_id);
        }

        let response: NoteCreateResponse = self
            .client
            .post(format!("https://{}/api/notes/create", CONFS.mk_endpnt))
            .json(&json)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    async fn create_segmented_note(
        &self,
        content: String,
        reply_id: Option<String>,
    ) -> anyhow::Result<()> {
        let mut reply_id = reply_id;
        let mut lines = Vec::new();
        let mut char_count = 0;

        for line in content.split('\n') {
            char_count += line.chars().count();

            if char_count > 600 {
                let response = self.create_note(lines.join("\n"), reply_id).await?;
                reply_id = Some(response.created_note.id);

                lines.clear();
                char_count = 0;
            }

            lines.push(line);
        }

        if char_count != 0 {
            self.create_note(lines.join("\n"), reply_id).await?;
        }

        Ok(())
    }
}

impl Interactor for InteractorMisskeyImpl {
    async fn announce(&self, content: String) -> anyhow::Result<()> {
        self.create_segmented_note(content, None).await?;
        Ok(())
    }

    async fn reply(&self, content: String, reply_id: String) -> anyhow::Result<()> {
        self.create_segmented_note(content, Some(reply_id)).await?;
        Ok(())
    }
}
