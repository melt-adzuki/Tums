#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]
#![feature(is_some_and)]

mod feat;
mod init;

use anyhow::Result;
use dotenv::dotenv;
use futures::StreamExt;
use init::init_api;
use log::*;
use std::env;

use crate::feat::{
    misskey::{api::*, body::*, streaming::*},
    uni::{api::*, utils::*},
};

#[tokio::main]
async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "info");
    dotenv().ok();
    env_logger::init();
    let api = init_api();

    let uni_now = api.list_all().await?.join("\n");
    info!("{}", uni_now);

    let instance = env::var("INSTANCE")?;
    let token = env::var("TOKEN")?;
    let timeline = env::var("TIMELINE").unwrap_or("localTimeLine".to_string());
    info!("instance: {}", instance);
    info!("token: {}", token);

    let streaming_url = get_stream_url(instance, token)?;

    let mut stream = connect_stream(streaming_url, timeline).await?;

    while let Some(msg) = stream.next().await {
        info!("{:?}", msg);
        let msg = msg.unwrap().to_string();
        if let Ok(deserialized) = serde_json::from_str::<StreamingBody>(msg.as_str()) {
            info!("{:?}", msg);
            let uni = deserialized.body.body.text;
            let lines: Vec<_> = uni.split('\n').map(|s| s.to_string()).collect();
            info!("is uni: {:?}", lines.is_uni());
            if lines.is_uni()? {
                let added_line = api.add_from_dust(lines).await?;
                info!("{:?}", added_line);

                let payload = NoteBody {
                    text: api.list_all().await?.join("\n"),
                    renote_id: None,
                    visibilty: None,
                    local_only: None,
                    cw: None,
                    comment: None,
                };

                payload.post().await?;
            }
        }
        // dbg!(msg);
    }

    Ok(())
}
