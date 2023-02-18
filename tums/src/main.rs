#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]
#![feature(is_some_and)]

use anyhow::Result;
use colored::Colorize;
use streaming::reciever::recieve;

mod confs;
mod consts;
mod domain;
mod drivers;
mod init;
mod log;
mod services;
mod streaming;
mod validation;

#[tokio::main]
async fn main() -> Result<()> {
    println!(
        "\n{}\n{}\n{}\n{}\n\nA Thoughtful Uni Management System\n",
        r"     _____ _   _ __  __ ___ ".bold().blue(),
        r"    |_   _| | | |  \/  / __|".bold().blue(),
        r"      | | | |_| | |\/| \__ \".bold().blue(),
        r"      |_|  \___/|_|  |_|___/".bold().blue()
    );

    log!("BOOT" -> "Starting up...".cyan());

    recieve().await?;
    Ok(())
}
