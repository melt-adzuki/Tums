use crate::domain::{interactor::Interactor, uni::UniRepository};

pub(crate) struct Service<T, U>
where
    T: UniRepository,
    U: Interactor,
{
    pub(crate) uni_repo: T,
    pub(crate) interactor: U,
}
