use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

use crate::{
    drivers::{interactor_test::InteractorTestImpl, uni_repository_mdb::UniRepositoryMdbDriver},
    services::service::Service,
};

pub(crate) static SERVICE: Lazy<Service<UniRepositoryMdbDriver, InteractorTestImpl>> =
    Lazy::new(|| Service {
        uni_repo: UniRepositoryMdbDriver::new(),
        interactor: InteractorTestImpl::new(),
    });

pub(crate) static TOKIO_RUNTIME: Lazy<Runtime> = Lazy::new(|| Runtime::new().unwrap());
