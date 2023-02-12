use anyhow::Result;

use crate::domain::{interactor::Interactor, uni::UniRepository};

use super::service::Service;

impl<T, U> Service<T, U>
where
    T: UniRepository,
    U: Interactor,
{
    /// 文字数制限に収まる範囲で最新のウニを取得し、全体にアナウンスします。
    pub(crate) async fn show_uni(&self) -> Result<()> {
        let mut unis = self.uni_repo.list().await?;
        unis.sort_by(|a, b| b.date.cmp(&a.date));

        let mut char_count = 0;
        let mut short_unis = Vec::new();

        for uni in unis {
            char_count += uni.content.chars().count();

            if char_count > 2700 {
                break;
            }

            short_unis.push(uni);
        }

        short_unis.sort_by(|a, b| a.pos.cmp(&b.pos));

        let content = short_unis
            .into_iter()
            .map(|u| u.content)
            .collect::<Vec<_>>()
            .join("\n");

        self.interactor.announce(content).await?;
        Ok(())
    }
}
