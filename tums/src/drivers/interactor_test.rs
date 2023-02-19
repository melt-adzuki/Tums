use anyhow::Result;

use crate::domain::interactor::Interactor;

pub(crate) struct InteractorTestImpl;

impl InteractorTestImpl {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl Interactor for InteractorTestImpl {
    async fn announce(&self, content: String) -> Result<()> {
        println!("\n--- アナウンスメント ---\n\n{}", content);
        Ok(())
    }

    async fn reply(&self, content: String, reply_id: String) -> Result<()> {
        println!("\n--- {} へのリプライ ---\n\n{}", reply_id, content);
        Ok(())
    }
}
