use std::sync::Arc;

use crate::uni_api::Api;
use anyhow::*;
use chrono::{DateTime, Utc};
use futures::{lock::Mutex, stream, try_join, StreamExt};
use mongodb::{
    bson::{doc, from_bson, oid::ObjectId, serde_helpers::chrono_datetime_as_bson_datetime, Bson},
    Collection,
};
use serde::{Deserialize, Serialize};
use similar::{ChangeTag, TextDiff};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct UniElem {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    content: String,
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    added_date: DateTime<Utc>,
    pos: i32,
}

pub(crate) struct Uni {
    pub(crate) collection: Collection<UniElem>,
}

impl Api for Uni {
    async fn from_dust(&self, s: Vec<String>) -> Result<Vec<String>> {
        let base = self.list_all().await?;

        let base = base.iter().map(|s| s.as_str()).collect::<Vec<_>>();
        let s = s.iter().map(|s| s.trim()).collect::<Vec<_>>();

        let diff = TextDiff::from_slices(base.as_slice(), s.as_slice());

        // Debug
        // for change in diff.iter_all_changes() {
        //     let sign = match change.tag() {
        //         ChangeTag::Delete => "-",
        //         ChangeTag::Insert => "+",
        //         ChangeTag::Equal => " ",
        //     };
        //     print!("{} {} {} {}", change.old_index().unwrap_or(0), sign, change.new_index().unwrap_or(0), change);
        // }

        let lost_line_counter = Arc::new(Mutex::new(0));

        let new_lines = stream::iter(diff.iter_all_changes())
            .filter(|c| {
                let c = *c;
                let counter = Arc::clone(&lost_line_counter);
                async move {
                    if c.tag() == ChangeTag::Delete {
                        *counter.lock().await += 1;
                    }
                    c.tag() == ChangeTag::Insert && c.to_string_lossy().trim().len() >= 4
                }
            })
            .filter_map(|c| {
                let c = c;
                let counter = Arc::clone(&lost_line_counter);
                async move {
                    let count = *counter.lock().await;
                    c.new_index()
                        .map(|index| (c.to_string_lossy().to_string(), index + 1 + count))
                }
            })
            .collect::<Vec<_>>()
            .await;

        for line in new_lines.iter() {
            let content = line.0.to_string();
            let pos = line.1 as i32;

            // let counter = Arc::clone(&lost_line_counter);
            // let count = counter.try_lock().unwrap().to_owned();
            // println!("{} + {} = {} --- {}", pos - (count as i32), count, pos, content);
            self.add(content, pos).await?;
        }

        let lines_added = new_lines
            .iter()
            .map(|line| line.0.to_string())
            .collect::<Vec<_>>();

        Ok(lines_added)
    }

    async fn list_all(&self) -> Result<Vec<String>> {
        let pipeline = vec![doc! {
            "$sort": { "pos": 1 },
        }];

        let cursor = self.collection.aggregate(pipeline, None).await?;

        let lines = cursor
            .filter_map(|d| async { d.ok() })
            .filter_map(|d| async { from_bson::<UniElem>(Bson::Document(d)).ok() })
            .map(|e| e.content.trim().to_string())
            .collect::<Vec<_>>()
            .await;

        Ok(lines)
    }

    async fn list_short(&self) -> Result<Vec<String>> {
        let pipeline = vec![doc! {
            "$sort": { "added_date": -1 },
        }];

        let mut cursor = self.collection.aggregate(pipeline, None).await?;

        let mut vec: Vec<UniElem> = Vec::new();
        let mut char_count = 0;

        while let Some(d) = cursor.next().await {
            let d = d?;
            let e = {
                let mut e = from_bson::<UniElem>(Bson::Document(d))?;
                e.content = e.content.trim().to_string();
                e
            };
            let c = e.content.chars().count();

            char_count += c;

            if char_count > 2700 {
                break;
            }

            vec.push(e);
        }

        vec.sort_by(|a, b| a.pos.cmp(&b.pos));

        let lines = vec
            .iter()
            .map(|e| e.content.to_string())
            .collect::<Vec<_>>();

        Ok(lines)
    }

    async fn add(&self, content: String, pos: i32) -> Result<()> {
        let last_pos = self.collection.count_documents(None, None).await? as i32;
        let query = doc! { "pos": { "$gte": pos, "$lte": last_pos } };
        let update = doc! { "$inc": { "pos": 1 } };
        self.collection.update_many(query, update, None).await?;

        self.collection
            .insert_one(
                UniElem {
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
