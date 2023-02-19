use anyhow::Result;

use crate::{
    domain::{
        interactor::{Interactor, YesNo},
        uni::UniRepository,
    },
    log,
};

use super::service::Service;

impl<T, U> Service<T, U>
where
    T: UniRepository,
    U: Interactor,
{
    /// 指定された位置の思慮深いウニを削除するか確認し、実行します。
    pub(crate) async fn remove_uni(&self, pos: i32, reply_id: String) -> Result<()> {
        log!("!" -> "Attempting to remove an Uni!".red().bold());

        let removing_uni = self.uni_repo.get(pos).await?;

        let msg = format!(
            "以下の思慮深いウニを削除します。よろしいですか？\n{}: {}",
            removing_uni.pos, removing_uni.content
        );

        let res = self.interactor.ask_yes_no(msg, reply_id).await?;

        match res {
            YesNo::Yes(reply_id) => {
                self.uni_repo.remove(pos).await?;
                self.interactor
                    .reply("削除しました".to_string(), reply_id)
                    .await?;
            }
            YesNo::No(reply_id) => {
                self.interactor
                    .reply("中止しました".to_string(), reply_id)
                    .await?;
            }
        }

        Ok(())
    }
}
