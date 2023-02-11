use anyhow::Result;
use chrono::{DateTime, Utc};

pub struct Uni {
    pub(crate) content: String,
    pub(crate) date: DateTime<Utc>,
    pub(crate) pos: i32,
}

pub trait UniRepository {
    async fn list(&self) -> Result<Vec<Uni>>;
    async fn get(&self, pos: i32) -> Result<Uni>;
    async fn add(&self, content: String, pos: i32) -> Result<()>;
    async fn remove(&self, pos: i32) -> Result<()>;
    async fn swap(&self, pos_1: i32, pos_2: i32) -> Result<()>;
}
