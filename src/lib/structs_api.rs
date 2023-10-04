use serde::{Deserialize, Serialize};

use crate::structs::Direction;

fn default_none() -> Option<i64> {
    None
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiPost {
    pub id: i64,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiFrontpage {
    pub posts: Vec<ApiPost>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiPostPage {
    pub parent_context: Vec<ApiPost>,
    pub post: ApiPost,
    pub note: Option<ApiPost>,
    pub replies: Vec<ApiPost>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiVote {
    pub post_id: i64,
    #[serde(default = "default_none")]
    pub note_id: Option<i64>,
    pub direction: ApiDirection,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ApiDirection {
    Up,
    Neutral,
    Down,
}

impl ApiDirection {
    pub fn from(direction: Direction) -> ApiDirection {
        match direction {
            Direction::Up => ApiDirection::Up,
            Direction::Neutral => ApiDirection::Neutral,
            Direction::Down => ApiDirection::Down,
        }
    }

    pub fn to_direction(&self) -> Direction {
        match self {
            ApiDirection::Up => Direction::Up,
            ApiDirection::Neutral => Direction::Neutral,
            ApiDirection::Down => Direction::Down,
        }
    }
}
