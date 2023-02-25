#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]
#![feature(is_some_and)]

use std::{thread, time::Duration};

use anyhow::Result;
use colored::Colorize;
use streaming::reciever::recieve;

mod confs;
mod consts;
mod domain;
mod drivers;
mod entities;
mod exceptions;
mod init;
mod log;
mod services;
mod streaming;

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

    let mut retry_duration = Duration::from_secs(10);

    loop {
        match recieve().await {
            Ok(_) => {}
            Err(error) => log!("ERR!" | "{:#?}", error),
        };

        log!("INFO" -> format!("Retrying after {} seconds...", retry_duration.as_secs()).cyan().bold());
        thread::sleep(retry_duration);
        retry_duration *= 2;
    }
}
