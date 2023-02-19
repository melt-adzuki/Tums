use anyhow::Result;

pub(crate) trait Interactor {
    async fn announce(&self, content: String) -> Result<()>;
    async fn reply(&self, content: String, reply_id: String) -> Result<()>;
}
