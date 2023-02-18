use futures::{SinkExt, StreamExt};
use log::*;
use serde_json::json;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use url::Url;

use crate::{
    confs::CONFS,
    streaming::{
        body::{NoteBody, StreamingBody},
        router::route,
    },
};

pub(crate) async fn recieve() -> anyhow::Result<()> {
    let url = format!("wss://{}/streaming?i={}", CONFS.mk_endpnt, CONFS.mk_token);
    let url: Url = url.parse()?;

    let (stream, _) = connect_async(url).await?;
    info!("Connection established!");

    let (mut write, read) = stream.split();

    write
        .send({
            let message = json!({
                "type": "connect",
                "body": {
                    "channel": CONFS.mk_tlcat,
                    "id": "1",
                }
            });
            Message::Text(message.to_string())
        })
        .await?;

    info!("Channel connection request sent.");

    read.for_each(|message| async {
        let message = match move || -> anyhow::Result<String> {
            let message = message?.to_text()?.to_string();
            Ok(message)
        }() {
            Ok(message) if !message.is_empty() => message,
            Err(error) => {
                error!("{:#?}", error);
                return;
            }
            _ => return,
        };

        match serde_json::from_str::<StreamingBody>(message.as_str()) {
            Ok(deserialized) => {
                let note_body: NoteBody = deserialized.body.body;
                info!("Recieved a note:\n{:#?}", note_body);
                match route(note_body).await {
                    Ok(_) => (),
                    Err(error) => error!("{:#?}", error),
                };
            }
            Err(error) => {
                warn!("Deserialization failed:\n{:#?}", error);
            }
        }
    })
    .await;

    Ok(())
}
