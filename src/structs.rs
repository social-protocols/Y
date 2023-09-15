//! Various structs used all over

use serde::Serialize;

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
