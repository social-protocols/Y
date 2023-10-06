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
    upvotes_given_shown_this_note: i64,
    votes_given_shown_this_note: i64,
    upvotes_given_not_shown_this_note: i64,
    votes_given_not_shown_this_note: i64,
}

impl InformedTallyQueryResult {
    fn informed_tally(&self) -> InformedTally {
        InformedTally {
            post_id: self.post_id,
            note_id: self.note_id,
            given_not_shown_this_note: Tally{
                upvotes: self.upvotes_given_not_shown_this_note,
                total: self.votes_given_not_shown_this_note,
            },
            given_shown_note: Tally{
                upvotes: self.upvotes_given_shown_this_note,
                total: self.votes_given_shown_this_note,
            },
        } 
    }
}

#[derive(sqlx::FromRow, sqlx::Decode, Debug, Clone)]
pub struct InformedTally {
    pub post_id: i64,
    pub note_id: i64,
    
    given_not_shown_this_note: Tally,
    given_shown_note: Tally,
}



pub async fn find_nop_note(post_id: i64, pool: &SqlitePool) -> Result<(i64, f64, f64)> {

    // first, get table which has stats for this note, all subnotes, and all subnotes
    let query = r#"
        WITH children AS
        (
          SELECT 
            post_id
            , note_id
            , votes_given_shown_this_note
            , upvotes_given_shown_this_note
            , votes_given_not_shown_this_note
            , upvotes_given_not_shown_this_note
          FROM current_informed_tally p
          WHERE post_id = ?
          UNION ALL
          SELECT 
            p.post_id
            , p.note_id
            , p.votes_given_not_shown_this_note
            , p.upvotes_given_not_shown_this_note
            , p.votes_given_not_shown_this_note
            , p.upvotes_given_not_shown_this_note
          FROM children c
          INNER JOIN current_informed_tally p ON p.post_id = c.note_id
        )
        select * from children;
    "#;

    

    // execute the query and get a vector of InformedTally
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

    Ok(find_top_note_given_informed_tallies(post_id, &tallies_map))
}

fn find_top_note_given_informed_tallies(post_id: i64, tallies_map: &HashMap<i64, Vec<InformedTally>>) -> (i64, f64, f64) {

    let tallies = tallies_map.get(&post_id);

    if tallies.is_none() {
        println!("End recursion for {}", post_id);
        return (1.0,1.0);
    }

    let mut p_of_a_given_shown_top_note: f64 = 0.0; 
    let mut p_of_a_given_not_shown_top_note: f64 = 0.0; // todo: should this be the same across all notes?
    let mut top_note_id: i64 = 0

    for tally in tallies.unwrap().iter() {
        let p_of_a_given_not_shown_this_note = p_of_a_given_not_shown_b(tally.clone()).average;
        let p_of_a_given_shown_this_note = p_of_a_given_shown_b(tally.clone()).average;
        let delta = p_of_a_given_shown_this_note - p_of_a_given_not_shown_this_note;

        let (subnote_id, p_of_b_given_shown_top_subnote, p_of_b_given_not_shown_top_subnote) = find_top_note_given_informed_tallies(tally.note_id, &tallies_map);

        let a = p_of_a_given_not_shown_this_note 
            + delta * p_of_b_given_shown_top_subnote/p_of_b_given_not_shown_top_subnote;

        if (a - p_of_a_given_not_shown_this_note).abs() > (p_of_a_given_shown_top_note - p_of_a_given_not_shown_top_note).abs() {
            p_of_a_given_shown_top_note = a;
            p_of_a_given_not_shown_top_note = p_of_a_given_not_shown_this_note; 
            top_note_id = tally.note_id
        }
    };

    (top_note_id, p_of_a_given_shown_top_note, p_of_a_given_not_shown_top_note)

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
    global_prior().bayesian_average(tally.given_not_shown_this_note)
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
              , upvotes_given_shown_this_note
              , votes_given_shown_this_note
              , upvotes_given_not_shown_this_note
              , votes_given_not_shown_this_note
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








