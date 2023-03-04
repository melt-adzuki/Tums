#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]
#![feature(is_some_and)]

use std::time::Duration;

use colored::Colorize;
use futures::lock::Mutex;
use once_cell::sync::Lazy;
use streaming::reciever::recieve;
use tokio::time;

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
async fn main() {
    println!(
        "\n{}\n{}\n{}\n{}\n\nA Thoughtful Uni Management System\n",
        r"     _____ _   _ __  __ ___ ".bold().blue(),
        r"    |_   _| | | |  \/  / __|".bold().blue(),
        r"      | | | |_| | |\/| \__ \".bold().blue(),
        r"      |_|  \___/|_|  |_|___/".bold().blue()
    );

    log!("BOOT" -> "Starting up...".cyan());

    static RETRY_COUNTER: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(0));

    tokio::spawn(async {
        let mut interval = time::interval(Duration::from_secs(3600));

        loop {
            interval.tick().await;
            *RETRY_COUNTER.lock().await = 0;
        }
    });

    loop {
        match recieve().await {
            Ok(_) => {}
            Err(error) => log!("ERR!" | "{:#?}", error),
        };

        let mut counter = RETRY_COUNTER.lock().await;

        if *counter >= 10 {
            break;
        }

        log!("INFO" -> "Retrying after 10 seconds...".cyan().bold());
        log!(
            "INFO" | "Remaining life(s): {}",
            (10 - *counter).to_string().red().bold()
        );

        time::sleep(Duration::from_secs(10)).await;
        *counter += 1;
    }

    log!("ERR!" -> "Too many retries, exiting...".red().bold());
}
