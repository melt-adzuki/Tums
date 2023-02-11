use anyhow::Result;

use crate::domain::models::interact::{InteractRepository, YesNo};

pub(crate) struct InteractRepositoryTestDriver {}

impl InteractRepositoryTestDriver {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl InteractRepository for InteractRepositoryTestDriver {
    async fn announce(&self, content: String) -> Result<()> {
        println!("\n--- アナウンスメント ---\n\n{}", content);
        Ok(())
    }

    async fn reply(&self, content: String, reply_id: String) -> Result<()> {
        todo!()
    }

    async fn ask_yes_no(&self, content: String, reply_id: String) -> Result<YesNo> {
        todo!()
    }
}
