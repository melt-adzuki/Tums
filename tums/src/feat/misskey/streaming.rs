use anyhow::*;
use futures_util::SinkExt;
use log::*;
use serde_json::json;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use url::Url;
use uuid::Uuid;

pub(crate) fn get_stream_url(instance: String, token: String) -> Result<Url> {
    let url = Url::parse(&format!("wss://{instance}/streaming?i={token}"))?;
    info!("url parsed: {}", url.to_string());

    Ok(url)
}

pub(crate) async fn connect_stream(
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
