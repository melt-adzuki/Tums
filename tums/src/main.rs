#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]

mod init;
mod uni;
mod uni_api;

use anyhow::Result;
use dotenv::dotenv;
use futures_util::{SinkExt, StreamExt};
use init::init_api;
use log::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use uni_api::Api;
use url::Url;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
struct StreamingResponse {
    #[serde(rename = "type")]
    streaming_type: String,
    body: Body,
}

#[derive(Serialize, Deserialize, Debug)]
struct Body {
    id: String,
    #[serde(rename = "type")]
    channel_type: String,
    body: NoteBody,
}

#[derive(Serialize, Deserialize, Debug)]
struct NoteBody {
    #[serde(rename = "renoteId")]
    renote_id: Option<String>,
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
enum Visibilty {
    Public,
    Home,
    Followers,
}

#[derive(Serialize, Deserialize, Debug)]
struct NotePayload {
    text: String,
    visibilty: Option<Visibilty>,
    local_only: Option<bool>,
    cw: Option<bool>,
    comment: Option<String>,
}

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
        if let Ok(deserialized) = serde_json::from_str::<StreamingResponse>(msg.as_str()) {
            info!("{:?}", msg);
            let uni = deserialized.body.body.text;
            info!("is uni: {:?}", is_uni(uni.to_string()));
            if is_uni(uni.to_string())? {
                let uni_lines: Vec<String> = uni.split('\n').map(|s| s.into()).collect();
                // info!("{:?}", uni_lines);

                let added_line = api.from_dust(uni_lines).await?;
                info!("{:?}", added_line);

                let payload = NotePayload {
                    text: api.list_all().await?.join("\n"),
                    visibilty: None,
                    local_only: None,
                    cw: None,
                    comment: None,
                };

                post_note(payload).await?;
            }
        }
        // dbg!(msg);
    }

    Ok(())
}

fn get_stream_url(instance: String, token: String) -> Result<Url> {
    let url = Url::parse(&format!("wss://{instance}/streaming?i={token}"))?;
    info!("url parsed: {}", url.to_string());

    Ok(url)
}

async fn connect_stream(
    url: Url,
    timeline: String,
) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>> {
    let (mut stream, _) = connect_async(url).await?;
    info!("connected");

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    let id = Uuid::new_v4().as_hyphenated().to_string();
    let msg_json = json!({
        "type": "connect",
        "body": {
            "channel": timeline,
            "id": id
        }
    });
    info!("{}", msg_json.to_string());

    stream.send(Message::Text(msg_json.to_string())).await?;
    info!("channel connected");

    Ok(stream)
}

fn is_uni(text: String) -> Result<bool> {
    if !text.starts_with("思慮深いウニ「俺が思うに...」")
        && !text.ends_with("大気圏に突入する犬「ウオオオオオオオオオオオオオオオオオオオオオオオオ")
    {
        return Ok(false);
    }

    let lines: Vec<_> = text.split('\n').collect();
    #[allow(clippy::never_loop)]
    for line in lines[1..(lines.len() - 1)].iter() {
        let reg = Regex::new(r"^(\S|\s)+「(\S|\s)+」$")?;

        if reg.is_match(line) {
            continue;
        }

        return Ok(false);
    }

    Ok(true)
}

async fn post_note(payload: NotePayload) -> Result<()> {
    // いい感じ™の処理
    let body = serde_json::to_string(&payload)?;

    info!("{}", body);

    Ok(())
}
