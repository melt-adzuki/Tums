use serde_json::json;

use crate::{confs::CONFS, domain::interactor::Interactor, log};

pub(crate) struct InteractorMisskeyImpl {
    client: reqwest::Client,
}

impl InteractorMisskeyImpl {
    pub(crate) fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

impl Interactor for InteractorMisskeyImpl {
    async fn announce(&self, content: String) -> anyhow::Result<()> {
        log!(
            "INFO" | "To {} >>> {}",
            "Timeline".yellow().bold(),
            "Creating a note...".cyan()
        );

        self.client
            .post(format!("https://{}/api/notes/create", CONFS.mk_endpnt))
            .json(&json!({
                "i": CONFS.mk_token,
                "text": content,
                "visibility": "home",
            }))
            .send()
            .await?;

        Ok(())
    }

    async fn reply(&self, content: String, reply_id: String) -> anyhow::Result<()> {
        log!(
            "INFO" | "To {} >>> {}",
            reply_id.green().bold(),
            "Creating a note...".cyan()
        );

        self.client
            .post(format!("https://{}/api/notes/create", CONFS.mk_endpnt))
            .json(&json!({
                "i": CONFS.mk_token,
                "text": content,
                "replyId": reply_id,
                "visibility": "home",
            }))
            .send()
            .await?;

        Ok(())
    }
}
