use crate::{
    confs::CONFS,
    domain::uni::{Uni, UniRepository},
};
use anyhow::*;
use chrono::{DateTime, Utc};
use futures::{try_join, StreamExt};
use mongodb::{
    bson::{doc, from_bson, oid::ObjectId, serde_helpers::chrono_datetime_as_bson_datetime, Bson},
    options::{ClientOptions, Credential},
    Client, Collection,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct UniBson {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    content: String,
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    added_date: DateTime<Utc>,
    pos: i32,
}

pub(crate) struct UniRepositoryMdbDriver {
    collection: Collection<UniBson>,
}

impl UniRepositoryMdbDriver {
    pub(crate) fn new() -> Self {
        let db = {
            let client_options = ClientOptions::builder()
                .hosts(vec![CONFS.db_host.parse().unwrap()])
                .credential(
                    Credential::builder()
                        .username(CONFS.db_username.to_string())
                        .password(CONFS.db_password.to_string())
                        .build(),
                )
                .build();

            let client = Client::with_options(client_options).unwrap();
            client.database("thoughtful_uni")
        };

        Self {
            collection: db.collection::<UniBson>("unis"),
        }
    }
}

impl UniRepository for UniRepositoryMdbDriver {
    async fn list(&self) -> Result<Vec<Uni>> {
        let pipeline = vec![doc! {
            "$sort": { "pos": 1 },
        }];

        let cursor = self.collection.aggregate(pipeline, None).await?;

        let unis = cursor
            .filter_map(|d| async { d.ok() })
            .filter_map(|d| async { from_bson::<UniBson>(Bson::Document(d)).ok() })
            .map(|b| Uni {
                content: b.content.trim().to_string(),
                date: b.added_date,
                pos: b.pos,
            })
            .collect::<Vec<_>>()
            .await;

        Ok(unis)
    }

    async fn get(&self, pos: i32) -> Result<Uni> {
        todo!()
    }

    async fn add(&self, content: String, pos: i32) -> Result<()> {
        let last_pos = self.collection.count_documents(None, None).await? as i32;
        let query = doc! { "pos": { "$gte": pos, "$lte": last_pos } };
        let update = doc! { "$inc": { "pos": 1 } };
        self.collection.update_many(query, update, None).await?;

        self.collection
            .insert_one(
                UniBson {
                    id: None,
                    content: content.trim().to_string(),
                    added_date: Utc::now(),
                    pos,
                },
                None,
            )
            .await?;

        Ok(())
    }

    async fn remove(&self, pos: i32) -> Result<()> {
        let last_pos = self.collection.count_documents(None, None).await? as i32;

        self.collection
            .delete_one(doc! { "pos": pos }, None)
            .await?;

        let query = doc! { "pos": { "$gt": pos, "$lte": last_pos } };
        let update = doc! { "$inc": { "pos": -1 } };
        self.collection.update_many(query, update, None).await?;

        Ok(())
    }

    async fn swap(&self, pos_1: i32, pos_2: i32) -> Result<()> {
        let filter = doc! { "pos": { "$in": [pos_1, pos_2] } };
        let cursor = self.collection.find(filter, None).await?;

        let vec = cursor
            .filter_map(|d| async { d.ok() })
            .collect::<Vec<_>>()
            .await;

        try_join!(
            self.collection.find_one_and_update(
                doc! { "_id": vec[0].id },
                doc! { "$set": { "pos": vec[1].pos } },
                None
            ),
            self.collection.find_one_and_update(
                doc! { "_id": vec[1].id },
                doc! { "$set": { "pos": vec[0].pos } },
                None
            ),
        )?;

        Ok(())
    }
}
