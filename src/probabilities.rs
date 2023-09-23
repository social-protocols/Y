// use crate::structs::Direction;
// use crate::structs::Direction::Down;
// use crate::structs::Direction::Up;
use anyhow::Result;
use sqlx::SqlitePool;
use std::fmt;

use crate::db;

const WEIGHT_CONSTANT: f64 = 2.3;

fn global_prior() -> BetaDistribution {
    BetaDistribution{average: 0.5, weight: WEIGHT_CONSTANT}
}

const EMPTY_TALLY: Tally = Tally{upvotes: 0, total: 0};

#[derive(sqlx::FromRow, sqlx::Decode, Debug, Clone)]
pub struct Tally {
    pub upvotes: i64,
    pub total: i64,
}

#[derive(Debug, Clone)]
pub struct BetaDistribution {
    pub average: f64,
    pub weight: f64,
}

impl BetaDistribution {
    fn from_alpha_beta(alpha: f64, beta: f64) -> Self {
        Self {
            average: alpha / (alpha + beta),
            weight: alpha + beta,
        }
    }

    fn bayesian_average(self, tally: Tally) -> Self {
        Self {
            average: (self.average * WEIGHT_CONSTANT + tally.upvotes as f64) / (WEIGHT_CONSTANT + tally.total as f64),
            weight: WEIGHT_CONSTANT + tally.total as f64,
        }
    }

}

impl fmt::Display for BetaDistribution {
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.average, self.weight)
        // write!("this is a Beta distribution")
    }
}

#[derive(sqlx::FromRow, sqlx::Decode, Debug, Clone)]
pub struct InformedTallyQueryResult {
    pub post_id: i64,
    pub note_id: i64,
    upvotes_given_shown_note: i64,
    votes_given_shown_note: i64,
    upvotes_given_not_shown_note: i64,
    votes_given_not_shown_note: i64,
}

#[derive(sqlx::FromRow, sqlx::Decode, Debug, Clone)]
pub struct InformedTally {
    pub post_id: i64,
    pub note_id: i64,
    
    given_not_shown_note: Tally,
    given_shown_note: Tally,
}

pub async fn informed_p_of_a(post_id: i64, pool: &SqlitePool) -> Result<BetaDistribution> {

    let top_note = db::get_top_note(post_id, pool).await?;

    Ok(match top_note {
        // if there is no top note, it means there are no votes
        None => global_prior(),
        Some(note) => {
            let tally = informed_tally(post_id, note.id, pool).await?;

            match tally {
                None => global_prior(),
                Some(t) => p_of_a_given_shown_b(t),
            }
        },
    })
}

// estimate P(A|not seen B) (preinformed).
// this is the prior for P(A|seen B)
// this is the belief in A of users who have not been exposed to B
// so we must exclude votes given B, but not votes given other notes
fn p_of_a_given_not_shown_b(tally: InformedTally) -> BetaDistribution {
    global_prior().bayesian_average(tally.given_not_shown_note)
}

fn p_of_a_given_shown_b(tally: InformedTally) -> BetaDistribution {
    let prior = p_of_a_given_not_shown_b(tally.clone());

    prior.bayesian_average(tally.given_shown_note) 
}

pub async fn informed_tally(
    post_id: i64,
    note_id: i64,
    pool: &SqlitePool,
) -> Result<Option<InformedTally>> {
    let optional_tally = sqlx::query_as::<_, InformedTallyQueryResult>(
        "select 
            post_id
            , note_id
              , upvotes_given_shown_note
              , votes_given_shown_note
              , upvotes_given_not_shown_note
              , votes_given_not_shown_note
              -- , upvotes_given_upvoted_note
              -- , votes_given_upvoted_note
              -- , upvotes_given_downvoted_note
              -- , votes_given_downvoted_note
        from current_informed_tally where post_id = ? and note_id = ?",
    )
    .bind(post_id)
    .bind(note_id)
    .fetch_optional(pool)
    .await?;


    Ok(optional_tally.map(|tally| 
            InformedTally{
                post_id: tally.post_id,
                note_id: tally.note_id,
                given_not_shown_note: Tally{
                    upvotes: tally.upvotes_given_not_shown_note,
                    total: tally.votes_given_not_shown_note,
                },
                given_shown_note: Tally{
                    upvotes: tally.upvotes_given_shown_note,
                    total: tally.votes_given_shown_note,
                },
            }
        
    ))
}








