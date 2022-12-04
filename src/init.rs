use mongodb::{Client, options::{ClientOptions, Credential}};

use crate::{uni::{Uni, UniElem}, api::Api};


pub(crate) fn init_api() -> impl Api {
    let db = {
        let client_options = ClientOptions::builder()
            .credential(Credential::builder()
                .username("root".to_string())
                .password("password".to_string())
                .build())
            .build();

        let client = Client::with_options(client_options).unwrap();
        client.database("thoughtful_uni")
    };

    Uni {
        collection: db.collection::<UniElem>("unis")
    }
}
