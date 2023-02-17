#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]
#![feature(is_some_and)]

use std::env;

use anyhow::Result;
use log::info;
use streaming::reciever::recieve;

use crate::confs::CONFS;

mod confs;
mod consts;
mod domain;
mod drivers;
mod init;
mod services;
mod streaming;
mod validation;

fn main() -> Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    info!("\n{:#?}", *CONFS);

    recieve()?;
    Ok(())
}
