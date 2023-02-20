use futures::{SinkExt, StreamExt};
use serde_json::json;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use url::Url;

use crate::{
    confs::CONFS,
    entities::{StreamingBody, User},
    log,
    streaming::router::route,
};

pub(crate) async fn recieve(me: &User) -> anyhow::Result<()> {
    log!("BOOT" -> "Connecting to the stream...".cyan());

    let url = format!("wss://{}/streaming?i={}", CONFS.mk_endpnt, CONFS.mk_token);
    let url: Url = url.parse()?;

    let (stream, _) = connect_async(url).await?;
    log!("BOOT" -> "Connection established!".green());

    let (mut write, read) = stream.split();

    write
        .send({
            let message = json!({
                "type": "connect",
                "body": {
                    "channel": "main",
                    "id": "1",
                }
            });
            Message::Text(message.to_string())
        })
        .await?;

    write
        .send({
            let message = json!({
                "type": "connect",
                "body": {
                    "channel": CONFS.mk_tlcat,
                    "id": "2",
                }
            });
            Message::Text(message.to_string())
        })
        .await?;

    log!("BOOT" -> "Ready!".green().bold());

    read.for_each(|message| async {
        let message = match move || -> anyhow::Result<String> {
            let message = message?.to_text()?.to_string();
            Ok(message)
        }() {
            Ok(message) if !message.is_empty() => message,
            Err(error) => {
                log!("ERR!" | "{:#?}", error);
                return;
            }
            _ => return,
        };

        let streaming_body: StreamingBody =
            match serde_json::from_str::<StreamingBody>(message.as_str()) {
                Ok(deserialized) => deserialized,
                Err(_) => return,
            };

        log!(
            "INFO" | "From {} stream <<< {}: {}",
            match streaming_body.body.id.as_str() {
                "1" => "main".green(),
                "2" => "Timeline".yellow(),
                _ => "unknown".red(),
            }
            .bold(),
            "A message recieved".bright_blue(),
            streaming_body.body.body.id
        );

        match route(me, streaming_body).await {
            Ok(_) => {}
            Err(error) => log!("ERR!" | "{:#?}", error),
        };
    })
    .await;

    Ok(())
}
