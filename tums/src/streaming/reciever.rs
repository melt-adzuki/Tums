use log::*;
use serde_json::json;
use ws::{connect, Message};

use crate::{
    confs::CONFS,
    streaming::{
        body::{NoteBody, StreamingBody},
        router::route,
    },
};

pub(crate) fn recieve() -> anyhow::Result<()> {
    let url = format!("wss://{}/streaming?i={}", CONFS.mk_endpnt, CONFS.mk_token);

    connect(url, |out| {
        let channel = json!({
            "type": "connect",
            "body": {
                "channel": CONFS.mk_tlcat,
                "id": "1",
            }
        });

        out.send(channel.to_string()).unwrap();
        info!("Connection established!");

        move |msg: Message| {
            let msg = match msg.as_text() {
                Ok(text) => text,
                Err(error) => {
                    error!("{:#?}", error);
                    return Ok(());
                }
            };

            match serde_json::from_str::<StreamingBody>(msg) {
                Ok(deserialized) => {
                    let note_body: NoteBody = deserialized.body.body;
                    info!("Recieved a note:\n{:#?}", note_body);
                    match route(note_body) {
                        Ok(_) => (),
                        Err(error) => error!("{}", error.to_string()),
                    };
                }
                Err(error) => {
                    warn!("Deserialization failed:\n{:#?}", error);
                }
            }

            Ok(())
        }
    })?;

    Ok(())
}
