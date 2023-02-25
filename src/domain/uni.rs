use anyhow::Result;
use chrono::{DateTime, Utc};
use regex::Regex;

use crate::consts::{UNISTR_FIRST, UNISTR_LAST, UNISTR_SECOND};

pub struct Uni {
    pub(crate) content: String,
    pub(crate) date: DateTime<Utc>,
    pub(crate) pos: i32,
}

pub(crate) trait IsUni {
    fn is_uni(&self) -> bool;
}

pub trait UniRepository {
    async fn list(&self) -> Result<Vec<Uni>>;
    async fn get(&self, pos: i32) -> Result<Uni>;
    async fn add(&self, content: String, pos: i32) -> Result<()>;
    async fn remove(&self, pos: i32) -> Result<()>;
    async fn swap(&self, pos_1: i32, pos_2: i32) -> Result<()>;
}

impl IsUni for Vec<&str> {
    fn is_uni(&self) -> bool {
        self.first().is_some_and(|s| *s == UNISTR_FIRST)
            && self.get(1).is_some_and(|s| *s == UNISTR_SECOND)
            && self.last().is_some_and(|s| *s == UNISTR_LAST)
            && self[2..self.len() - 1]
                .iter()
                .all(|s| s.len() >= 4 && Regex::new(r".+「.+」").is_ok_and(|r| r.is_match(s)))
    }
}
