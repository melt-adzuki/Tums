#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]
#![feature(is_some_and)]

/*
use anyhow::Result;
use drivers::{interactor_test::InteractorTestImpl, uni_repository_mdb::UniRepositoryMdbDriver};
use services::service::Service;

mod confs;
mod domain;
mod drivers;
mod services;

#[tokio::main]
async fn main() -> Result<()> {
    let service = Service {
        uni_repo: UniRepositoryMdbDriver::new(),
        interactor: InteractorTestImpl::new(),
    };

    service.show_uni().await?;
    Ok(())
}
*/

mod confs;
mod streaming;

use anyhow::Result;
use log::*;
use std::env;

use crate::{confs::CONFS, streaming::reciever::start_recieving};

#[tokio::main]
async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    info!("\n{:#?}", *CONFS);
    start_recieving().await?;

    Ok(())
}
