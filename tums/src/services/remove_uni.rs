use anyhow::Result;

use crate::domain::{interactor::Interactor, uni::UniRepository};

use super::service::Service;

impl<T, U> Service<T, U>
where
    T: UniRepository,
    U: Interactor,
{
    /// 指定された位置にある思慮深いウニを削除します。
    pub(crate) async fn remove_uni(&self, pos: i32, reply_id: String) -> Result<()> {
        let removing_uni = self.uni_repo.get(pos).await?;
        self.uni_repo.remove(pos).await?;

        let message = format!(
            "以下の思慮深いウニを削除しました:\n\n{}. {}",
            removing_uni.pos, removing_uni.content
        );

        self.interactor.reply(message, reply_id).await?;
        Ok(())
    }
}
