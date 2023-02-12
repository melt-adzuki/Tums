use anyhow::Result;

pub(crate) trait Interactor {
    async fn announce(&self, content: String) -> Result<()>;
    async fn reply(&self, content: String, reply_id: String) -> Result<()>;
    async fn ask_yes_no(&self, content: String, reply_id: String) -> Result<YesNo>;
}

pub(crate) enum YesNo {
    Yes(String),
    No(String),
}
