CREATE TABLE _sqlx_migrations (
    version BIGINT PRIMARY KEY,
    description TEXT NOT NULL,
    installed_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    success BOOLEAN NOT NULL,
    checksum BLOB NOT NULL,
    execution_time BIGINT NOT NULL
);
CREATE TABLE posts (
    id integer primary key -- row id
    , content text not null
    , parent_id integer references posts (id) on delete cascade on update cascade -- nullable
    , question_id integer references posts (id) on delete cascade on update cascade -- nullable
        -- if a question is a top-level question -> no parent and question_id = id
        -- if a question is a reply -> parent is the id of the post it replies to
        -- -- if it reuses a question -> that question is its question_id
        -- -- if it doesn't reuse a question -> question_id = id
    , created TIMESTAMP not null DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE users (
  id integer not null primary key, -- rowid
  secret text not null unique,
  created TIMESTAMP not null DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE vote_history (
    user_id not null references users (id) on delete cascade on update cascade
    , post_id not null references posts (id) on delete cascade on update cascade
    , note_id references posts (id) on delete cascade on update cascade
    , direction integer not null
    , created TIMESTAMP not null DEFAULT CURRENT_TIMESTAMP
);
CREATE VIEW current_informed_tally as 
with current_informed_votes as (
    SELECT
      user_id
      , post_id
      , note_id

      -- NOTE: direction will be the value of direction pulled from the same row that has max(created)
      -- https://www.sqlite.org/lang_select.html#bareagg
      , direction
      , max(created) AS created
    FROM vote_history
    where note_id is not null
    GROUP BY 1,2, 3

)
, informed_tally as (
  select 
    post_id
    , note_id
    , sum(case when direction = 1 then 1 else 0 end) as upvotes
    , count(*) as votes
  from current_informed_votes
  -- The latest vote might be zero, so in that case we don't return a record for this user and post
  where direction != 0
  group by 1, 2 
),  
first_votes_on_notes as (
  SELECT 
        user_id
        , post_id
        , note_id
        -- , direction
        , min(rowid) first_vote_on_this_note_rowid
  FROM vote_history
  WHERE note_id is not null
  GROUP BY 1, 2, 3
)
, votes_before_note as (
    select
      params.post_id as p_post_id
      , params.note_id as p_note_id
      , first_votes_on_notes.post_id as f_post_id 
      , first_votes_on_notes.post_id as f_note_id 
      , first_votes_on_notes.first_vote_on_this_note_rowid
      , vote_history.rowid
      , vote_history.*
      , case when 
          (first_vote_on_this_note_rowid is null or vote_history.rowid < first_vote_on_this_note_rowid)
          -- and direction == 1
        then true
        else null end before_note
    
      , params.upvotes as upvotes_given_shown_this_note
      , params.votes as votes_given_shown_this_note
    FROM 
       informed_tally params
       join vote_history using (post_id)
    LEFT OUTER JOIN first_votes_on_notes on (
       first_votes_on_notes.post_id = params.post_id
       and first_votes_on_notes.note_id = params.note_id
       and first_votes_on_notes.user_id = vote_history.user_id
    )
)
, last_votes_before_note as (
    select
        p_post_id as post_id
        , p_note_id as note_id
        , user_id
        , direction
        , created
        , upvotes_given_shown_this_note
        , votes_given_shown_this_note
        , max(created)
    from  votes_before_note
    where 
    before_note 
    group by 1, 2, 3
)
select
  post_id
  , note_id
  , sum(
    case when direction == 1
    then 1 
    else 0 end 
  ) as upvotes_given_not_shown_this_note
  , count(*) as votes_given_not_shown_this_note

  , upvotes_given_shown_this_note
  , votes_given_shown_this_note
from last_votes_before_note
group by 1,2
/* current_informed_tally(post_id,note_id,upvotes_given_not_shown_this_note,votes_given_not_shown_this_note,upvotes_given_shown_this_note,votes_given_shown_this_note) */;
CREATE VIEW current_tally as
select
  post_id
  , sum(case when direction = 1 then 1 else 0 end) as upvotes
  , count(*) as votes
  -- , sum(case when note_id is null and direction = 1 then 1 else 0 end) as upvotes_given_not_seen_any_note
  -- , sum(case when note_id is null then 1 else 0 end) as votes_given_not_seen_any_note
from current_vote 
group by 1
/* current_tally(post_id,upvotes,votes) */;
CREATE VIEW current_vote as
with latest as (
    SELECT
      user_id
      , post_id
      -- TODO: Check whether this is reliable behavior. From what I can tell, direction will be the direction from teh record
      -- corresponding to max(created), that it, it will be the value of the user's latest vote
      , direction
      , max(created) AS created
    FROM vote_history
    GROUP BY 1,2

-- The latest vote might be zero, so in that case we don't return a record for this user and post
) select * from latest where direction != 0
/* current_vote(user_id,post_id,direction,created) */;
CREATE VIEW probabilities_given_note as
with parameters as (
    select 
        .85 as prior 
        , 2 as priorWeight

)
, given_not_shown_this_note as (
    select 
        *
        , (cast(upvotes_given_not_shown_this_note + prior*priorWeight as float)) / (cast(votes_given_not_shown_this_note + priorWeight as float)) as p_given_not_shown_this_note
    from current_informed_tally join parameters
)
, given_shown_this_note as (
    select 
        post_id
        , note_id 
        , upvotes_given_not_shown_this_note 
        , votes_given_not_shown_this_note 
        , p_given_not_shown_this_note 
        , upvotes_given_shown_this_note 
        , votes_given_shown_this_note  
        , ( upvotes_given_shown_this_note + p_given_not_shown_this_note * priorWeight) / (cast(votes_given_shown_this_note + priorWeight as float)) as p_given_shown_this_note
    from given_not_shown_this_note
)
select 
    * 
    -- , max("p(A=1|vA,sB)")
    from given_shown_this_note
    -- group by post_id
/* probabilities_given_note(post_id,note_id,upvotes_given_not_shown_this_note,votes_given_not_shown_this_note,p_given_not_shown_this_note,upvotes_given_shown_this_note,votes_given_shown_this_note,p_given_shown_this_note) */;
