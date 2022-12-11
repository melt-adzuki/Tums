use mongodb::{
    options::{ClientOptions, Credential},
    Client,
};

use crate::{
    uni::{Uni, UniElem},
    uni_api::Api,
};
use dotenv::dotenv;
use std::env;

pub(crate) fn init_api() -> impl Api {
    dotenv().ok();
    let db_host = env::var("DB_HOST").unwrap_or("localhost:27017".to_string());

    let db = {
        let client_options = ClientOptions::builder()
            .hosts(vec![db_host.parse().unwrap()])
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
