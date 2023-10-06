// use crate::structs::Direction;
// use crate::structs::Direction::Down;
// use crate::structs::Direction::Up;
use anyhow::Result;
use sqlx::SqlitePool;
use std::fmt;

use crate::db;
use std::collections::HashMap;

const WEIGHT_CONSTANT: f64 = 2.3;

fn global_prior() -> BetaDistribution {
    BetaDistribution{average: 0.875, weight: WEIGHT_CONSTANT}
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

impl InformedTallyQueryResult {
    fn informed_tally(&self) -> InformedTally {
        InformedTally {
            post_id: self.post_id,
            note_id: self.note_id,
            given_not_shown_note: Tally{
                upvotes: self.upvotes_given_not_shown_note,
                total: self.votes_given_not_shown_note,
            },
            given_shown_note: Tally{
                upvotes: self.upvotes_given_shown_note,
                total: self.votes_given_shown_note,
            },
        } 
    }
}

#[derive(sqlx::FromRow, sqlx::Decode, Debug, Clone)]
pub struct InformedTally {
    pub post_id: i64,
    pub note_id: i64,
    
    given_not_shown_note: Tally,
    given_shown_note: Tally,
}



pub async fn hypothetical_p_of_a(post_id: i64, pool: &SqlitePool) -> Result<(f64, f64)> {

    // first, get table which has stats for this note, all subnotes, and all subnotes
    let query = r#"
        WITH children AS
        (
          SELECT 
            post_id
            , note_id
            , votes_given_shown_note
            , upvotes_given_shown_note
            , votes_given_not_shown_note
            , upvotes_given_not_shown_note
          FROM current_informed_tally p
          WHERE post_id = 1

          -- UNION ALL
          -- -- selects all posts that are a parent of something in c
          -- SELECT p.post_id, p.note_id, p.votes_given_not_shown_note, p.upvotes_given_not_shown_note, p.votes_given_not_shown_note, p.upvotes_given_not_shown_note
          -- FROM children c
          -- INNER JOIN current_informed_tally p ON p.post_id = c.note_id
        )
        select * from children;
    "#;

    

    // execute the query and get a vector of Votes
    let tallies: Vec<InformedTally> = sqlx::query_as::<_, InformedTallyQueryResult>(query)
        .bind(post_id)
        .fetch_all(pool)
        .await?
        .iter()
        .map(|result| result.informed_tally())
        .collect();
        // .iter();

    
    let mut tallies_map: HashMap<i64, Vec<InformedTally>> = HashMap::new();
   
    // TODO: somebody who actually understands Rust borrow checking rewrite this to avoid unecessarily cloning
    // the array each time we append to it. 
    for tally in tallies.iter() {
        let key = tally.post_id;
        let mut v = match tallies_map.get(&key) {
            None => Vec::new(),
            Some(vec) => vec.clone(),
        };
        v.push(tally.clone());
        tallies_map.insert(key, v);
    };

    Ok(hypothetical_p_of_a_recursive(post_id, &tallies_map))
}

pub fn hypothetical_p_of_a_recursive(post_id: i64, tallies_map: &HashMap<i64, Vec<InformedTally>>) -> (f64, f64) {

    let mut top_note_hypothetical_a: f64 = 0.0;
    let mut top_note_a_given_not_sb: f64 = 0.0; // todo: should this be the same across all notes?

    let tallies = tallies_map.get(&post_id);

    if tallies.is_none() {
        println!("End recursion for {}", post_id);
        return (1.0,1.0);
    }

    for tally in tallies.unwrap().iter() {
        let a_given_not_sb = p_of_a_given_not_shown_b(tally.clone()).average;
        let a_given_sb = p_of_a_given_shown_b(tally.clone()).average;
        let delta = a_given_sb - a_given_not_sb;

        let (hypothetical_b, b_given_not_sc) = hypothetical_p_of_a_recursive(tally.note_id, &tallies_map);
        let support = hypothetical_b/b_given_not_sc;

        let hypothetical_a = a_given_not_sb + delta * support;
        if (hypothetical_a - a_given_not_sb).abs() > (top_note_hypothetical_a - top_note_a_given_not_sb).abs() {
            top_note_hypothetical_a = hypothetical_a;
            top_note_a_given_not_sb = a_given_not_sb; 
        }
    };

    (top_note_hypothetical_a, top_note_a_given_not_sb)

    // let top_note = db::get_top_note(post_id, pool).await?;

    // Ok(match top_note {
    //     // if there is no top note, it means there are no votes
    //     None => global_prior(),
    //     Some(note) => {
    //         let tally = informed_tally(post_id, note.id, pool).await?;

    //         match tally {
    //             None => global_prior(),
    //             Some(t) => p_of_a_given_shown_b(t),
    //         }
    //     },
    // })
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

async fn informed_tally(
    post_id: i64,
    note_id: i64,
    pool: &SqlitePool,
) -> Result<Option<InformedTally>> {
    let optional_result = sqlx::query_as::<_, InformedTallyQueryResult>(
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

    Ok(optional_result.map(|result| result.informed_tally()))
}








