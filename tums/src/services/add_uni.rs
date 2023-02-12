use anyhow::Result;

use crate::domain::{interactor::Interactor, uni::UniRepository};

use super::service::Service;

impl<T, U> Service<T, U>
where
    T: UniRepository,
    U: Interactor,
{
    /// 指定された位置に思慮深いウニを追加した後、全体に結果をアナウンスします。
    pub(crate) async fn add_uni(&self, content: String, pos: i32) -> Result<()> {
        self.uni_repo.add(content, pos).await?;

        let new_unis = self.uni_repo.list().await?;
        let new_unis = new_unis
            .into_iter()
            .map(|u| u.content)
            .collect::<Vec<_>>()
            .join("\n");

        self.interactor.announce(new_unis).await?;
        Ok(())
    }
}
