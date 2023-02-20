use anyhow::{Context, Result};
use rand::seq::SliceRandom;

use crate::domain::{interactor::Interactor, uni::UniRepository};

use super::service::Service;

const SERIFS: [&str; 5] = [
    "ฅ(^•ω•^ฅ",
    "(=^・・^=)",
    "(ฅ >ω< ฅ)",
    "(=^･ω･^=)",
    "(=^･ｪ･^=)",
];

impl<T, U> Service<T, U>
where
    T: UniRepository,
    U: Interactor,
{
    pub(crate) async fn cat(&self, reply_id: String) -> Result<()> {
        let message = SERIFS
            .choose(&mut rand::thread_rng())
            .context("No serifs found.")?
            .to_string();

        self.interactor.reply(message, reply_id).await?;
        Ok(())
    }
}
