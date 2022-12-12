use mongodb::{
    options::{ClientOptions, Credential},
    Client,
};

use crate::feat::uni::{
    api::Api,
    db::{Uni, UniElem},
};
use dotenv::dotenv;
use std::env;

pub(crate) fn init_api() -> impl Api {
    dotenv().ok();

    let db = {
        let host = env::var("DB_HOST").unwrap_or("localhost:27017".to_string());

        let client_options = ClientOptions::builder()
            .hosts(vec![host.parse().unwrap()])
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
