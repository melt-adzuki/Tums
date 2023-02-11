use anyhow::Result;

use crate::domain::models::{
    interact::{InteractRepository, YesNo},
    uni::UniRepository,
};

use super::service::Service;

impl<T, U> Service<T, U>
where
    T: UniRepository,
    U: InteractRepository,
{
    /// 指定された位置の思慮深いウニを削除するか確認し、実行します。
    pub(crate) async fn remove_uni_service(&self, pos: i32, reply_id: String) -> Result<()> {
        let removing_uni = self.uni_repo.get(pos).await?;

        let msg = format!(
            "以下の思慮深いウニを削除します。よろしいですか？\n{}: {}",
            removing_uni.pos, removing_uni.content
        );

        let res = self.interact_repo.ask_yes_no(msg, reply_id).await?;

        match res {
            YesNo::Yes(reply_id) => {
                self.uni_repo.remove(pos).await?;
                self.interact_repo
                    .reply("削除しました".to_string(), reply_id)
                    .await?;
            }
            YesNo::No(reply_id) => {
                self.interact_repo
                    .reply("中止しました".to_string(), reply_id)
                    .await?;
            }
        }

        Ok(())
    }
}
