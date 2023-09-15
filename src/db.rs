//! Database access via sqlx

use anyhow::Result;
use sqlx::SqlitePool;

use crate::structs::{StatementStats, TargetSegment, User, Vote};

use crate::{
    highlight::{HIGHLIGHT_BEGIN, HIGHLIGHT_END},
    structs::{SearchResultStatement, Statement, VoteHistoryItem},
};
