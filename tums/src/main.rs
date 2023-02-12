#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]
#![feature(is_some_and)]

use anyhow::Result;
use drivers::{interactor_test::InteractorTestImpl, uni_repository_mdb::UniRepositoryMdbDriver};
use services::service::Service;

mod confs;
mod domain;
mod drivers;
mod services;

#[tokio::main]
async fn main() -> Result<()> {
    let service = Service {
        uni_repo: UniRepositoryMdbDriver::new(),
        interactor: InteractorTestImpl::new(),
    };

    service.show_uni().await?;
    Ok(())
}

/*
use anyhow::Result;
use futures::StreamExt;
use log::*;
use std::env;

use crate::{
    confs::CONFS,
    feat::{
        misskey::{api::*, body::*, streaming::*},
        uni::{api::*, utils::*},
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    let api = init_api();

    let uni_now = api.list_all().await?.join("\n");
    info!("{}", uni_now);

    info!("{:#?}", *CONFS);

    let streaming_url = get_stream_url(CONFS.mk_endpnt.to_string(), CONFS.mk_token.to_string())?;

    let mut stream = connect_stream(streaming_url, CONFS.mk_tlcat.to_string()).await?;

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
*/
