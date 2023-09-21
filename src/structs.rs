//! Various structs used all over

extern crate derive_more;
use derive_more::{Display};
use serde::{Deserialize, Serialize};

/// Representation of a user. Provides various methods to find & update them
#[derive(Serialize, sqlx::FromRow, Debug, Clone)]
pub struct User {
    pub id: i64,
    pub secret: String,
}

#[derive(Clone)]
pub struct PageMeta {
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Post {
    pub id: i64,
    pub content: String,
    pub parent_id: Option<i64>,
}

#[derive(PartialEq, Deserialize, Copy, Clone, Display)]
#[non_exhaustive]
pub enum Direction {
    Up = 1,
    None = 0,
    Down = -1,
}
