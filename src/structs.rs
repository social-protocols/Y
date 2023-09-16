//! Various structs used all over

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
}

#[derive(PartialEq, Deserialize, Copy, Clone)]
#[non_exhaustive]
pub enum Direction {
    Down = 1,
    None = 0,
    Up = -1,
}
