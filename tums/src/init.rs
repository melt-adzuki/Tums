use once_cell::sync::Lazy;

use crate::{
    drivers::{interactor_mk::InteractorMisskeyImpl, uni_repository_mdb::UniRepositoryMdbDriver},
    services::service::Service,
};

pub(crate) static REQWEST_CLIENT: Lazy<reqwest::Client> = Lazy::new(reqwest::Client::new);

pub(crate) static SERVICE: Lazy<Service<UniRepositoryMdbDriver, InteractorMisskeyImpl>> =
    Lazy::new(|| Service {
        uni_repo: UniRepositoryMdbDriver::new(),
        interactor: InteractorMisskeyImpl::new(),
    });
