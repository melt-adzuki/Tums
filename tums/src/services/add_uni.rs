use anyhow::Result;

use crate::domain::{interactor::Interactor, uni::UniRepository};

use super::service::Service;

impl<T, U> Service<T, U>
where
    T: UniRepository,
    U: Interactor,
{
    /// 指定された位置に思慮深いウニを追加します。
    pub(crate) async fn add_uni(&self, content: String, pos: i32, reply_id: String) -> Result<()> {
        self.uni_repo.add(content.clone(), pos).await?;

        let message = format!(
            "{}番目に以下の思慮深いウニを追加しました:\n\n{}",
            pos, content
        );

        self.interactor.reply(message, reply_id).await?;
        Ok(())
    }
}
