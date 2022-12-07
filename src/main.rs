#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]

mod uni;
mod api;
mod init;

use anyhow::*;
use api::Api;
use init::init_api;


#[tokio::main]
async fn main() -> Result<()> {
    let api = init_api();

    // let s = "abc\ndef\nghi";
    // let dust = s.split("\n").map(|s| s.to_string()).collect::<Vec<_>>();
    // let res = api.from_dust(dust).await?;

    // api.add("jkl".to_string(), 4).await?;
    
    // api.remove(5).await?;

    // api.swap(6, 3).await?;

    let res = api.list_all().await?;
    // let res = api.list_short().await?;

    println!("{}", res.join("\n"));

    Ok(())
}
