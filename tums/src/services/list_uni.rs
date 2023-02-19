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
    /// すべての思慮深いウニを、文字数制限ごとに分割して返信します。
    pub(crate) async fn list_uni(&self, reply_id: String) -> Result<()> {
        log!("INFO" -> "Listing Unis...".cyan());

        let unis = self.uni_repo.list().await?;

        let message = unis
            .into_iter()
            .map(|u| format!("{}. {}", u.pos, u.content))
            .collect::<Vec<_>>()
            .join("\n");

        let message = format!(
            "{}\n{}\n{}\n{}",
            UNISTR_FIRST, UNISTR_SECOND, message, UNISTR_LAST
        );

        self.interactor.reply(message, reply_id).await?;
        Ok(())
    }
}
