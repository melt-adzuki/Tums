use anyhow::Result;

use crate::{
    consts::{UNISTR_FIRST, UNISTR_LAST, UNISTR_SECOND},
    domain::{interactor::Interactor, uni::UniRepository},
    log,
};

use super::service::Service;

impl<T, U> Service<T, U>
where
    T: UniRepository,
    U: Interactor,
{
    /// ある程度の長さに収まる範囲で最新のウニを取得し、全体にアナウンスします。
    pub(crate) async fn show_uni(&self) -> Result<()> {
        log!("INFO" -> "Showing Unis...".cyan());

        let mut unis = self.uni_repo.list().await?;
        unis.sort_by(|a, b| b.date.cmp(&a.date));

        let mut char_count = [UNISTR_FIRST, UNISTR_SECOND, UNISTR_LAST]
            .into_iter()
            .map(|s| s.chars().count())
            .sum::<usize>();
        let mut short_unis = Vec::new();

        for uni in unis {
            char_count += uni.content.chars().count();

            if char_count > 500 {
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

        let message = format!(
            "{}\n{}\n{}\n{}\n",
            UNISTR_FIRST, UNISTR_SECOND, content, UNISTR_LAST
        );

        self.interactor.announce(message).await?;
        Ok(())
    }
}
