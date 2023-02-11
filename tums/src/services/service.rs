use crate::domain::models::{interact::InteractRepository, uni::UniRepository};

pub(crate) struct Service<T, U>
where
    T: UniRepository,
    U: InteractRepository,
{
    pub(crate) uni_repo: T,
    pub(crate) interact_repo: U,
}
