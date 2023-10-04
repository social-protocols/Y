//! Various structs used all over

extern crate derive_more;
use anyhow::{anyhow, Result};
use derive_more::Display;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
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

#[derive(Debug, PartialEq, Deserialize, Copy, Clone, FromPrimitive, Display, Serialize)]
pub enum Direction {
    Up = 1,
    Neutral = 0,
    Down = -1,
}

impl Direction {
    pub fn from(direction: i32) -> Result<Direction> {
        FromPrimitive::from_i32(direction).ok_or(anyhow!("Unknown direction value: {}", direction))
    }
}
