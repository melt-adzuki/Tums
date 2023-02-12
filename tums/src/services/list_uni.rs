use anyhow::Result;

use crate::domain::{interactor::Interactor, uni::UniRepository};

use super::service::Service;

impl<T, U> Service<T, U>
where
    T: UniRepository,
    U: Interactor,
{
    /// すべての思慮深いウニを、文字数制限ごとに分割して返信します。
    pub(crate) async fn list_uni(&self) -> Result<()> {
        todo!()
    }
}
