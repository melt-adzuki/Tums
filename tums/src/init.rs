use mongodb::{
    options::{ClientOptions, Credential},
    Client,
};

use crate::{
    confs::CONFS,
    feat::uni::{
        api::Api,
        db::{Uni, UniElem},
    },
};

pub(crate) fn init_api() -> impl Api {
    let db = {
        let client_options = ClientOptions::builder()
            .hosts(vec![CONFS.db_host.parse().unwrap()])
            .credential(
                Credential::builder()
                    .username("root".to_string())
                    .password("password".to_string())
                    .build(),
            )
            .build();

        let client = Client::with_options(client_options).unwrap();
        client.database("thoughtful_uni")
    };

    Uni {
        collection: db.collection::<UniElem>("unis"),
    }
}
