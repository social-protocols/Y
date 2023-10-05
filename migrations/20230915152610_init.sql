CREATE TABLE users (
  id integer not null primary key, -- rowid
  secret text not null unique,
  created TIMESTAMP not null DEFAULT CURRENT_TIMESTAMP
);

create table posts (
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

create table vote_history (
    user_id not null references users (id) on delete cascade on update cascade
    , post_id not null references posts (id) on delete cascade on update cascade
    , note_id references posts (id) on delete cascade on update cascade
    , direction integer not null
    , created TIMESTAMP not null DEFAULT CURRENT_TIMESTAMP
);



-- The current (latest) votes for all users on all posts
-- If the user has cleared their vote, no row is returned.
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
) select * from latest where direction != 0;


-- current_tally counts counts the latest votes, regardless of whether they are informed or not.
create view current_tally as
select
  post_id
  , sum(case when direction = 1 then 1 else 0 end) as upvotes
  , count(*) as votes
  -- , sum(case when note_id is null and direction = 1 then 1 else 0 end) as upvotes_given_not_seen_any_note
  -- , sum(case when note_id is null then 1 else 0 end) as votes_given_not_seen_any_note
from current_vote 
group by 1;


-- drop view if exists informed_vote;
-- CREATE VIEW current_vote as
-- with latest as (
--     SELECT
--       user_id
--       , post_id
--       , direction
--       , max(created) AS created
--     FROM vote_history
--     GROUP BY 1,2
-- ) select * from latest where direction != 0


drop view if exists current_informed_tally;
create view current_informed_tally as 
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
    
      , params.upvotes as upvotes_given_shown_note
      , params.votes as votes_given_shown_note
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
        , upvotes_given_shown_note
        , votes_given_shown_note
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
  ) as upvotes_given_not_shown_note
  , count(*) as votes_given_not_shown_note

  , upvotes_given_shown_note
  , votes_given_shown_note
from last_votes_before_note
group by 1,2;



-- drop view if exists current_informed_tally;
-- create view current_informed_tally as
-- with current_informed_votes as (
--     SELECT
--       user_id
--       , post_id
--       , note_id
--       -- TODO: Check whether this is reliable behavior. From what I can tell, direction will be the direction from teh record
--       -- corresponding to max(created), that it, it will be the value of the user's latest vote
--       , direction
--       , max(created) AS created
--     FROM vote_history
--     where note_id is not null
--     GROUP BY 1,2, 3

-- -- The latest vote might be zero, so in that case we don't return a record for this user and post
-- ),  
-- current_informed_tally as (
--   select * from current_informed_votes where direction != 0
-- )





-- select
--   A.post_id
--   , A.note_id

--   , count(*)                                            as votes_given_seen_note

--   , sum(case when A.direction = 1 then 1 else 0 end) as upvotes_given_seen_note

-- -----

--   , sum(case when A.direction = 1 
--               and B.direction = 1 then 1 else 0 end) as upvotes_given_upvoted_note

--   , sum(case when B.direction = 1 then 1 else 0 end) as votes_given_upvoted_note

--   , sum(case when A.direction = 1 
--               and B.direction = -1 then 1 else 0 end) as upvotes_given_downvoted_note

--   , sum(case when B.direction = -1 then 1 else 0 end) as votes_given_downvoted_note
-- from 
--     informed_votes A
--     left join current_vote B
--     on (A.note_id = B.post_id and A.user_id = B.user_id)
-- group by 1,2;





insert into posts(id, content, parent_id, question_id) values (1, "So, pregnant people can’t cross state lines to get abortions but guys like Kyle Rittenhouse can cross state lines to murder people. Seems fair.", null, null);
insert into posts(id, content, parent_id, question_id) values (2, "Kyle Rittenhouse was acquitted of murder charges. Clear video evidence showed he acted in self defense.", 1, null);
insert into posts(id, content, parent_id, question_id) values (3, "That trial was a sham. They were never going to convict.", 2, null);



insert into posts(id, content, parent_id, question_id) values (4, "Sudafed, Benadryl and most decongestants don’t work: FDA advisory panel https://trib.al/sJmOJBP", null, null);
insert into posts(id, content, parent_id, question_id) values (5, "This is misleading. Regular Benadryl is an antihistamine; it is not a decongestant. There is a Benadryl branded product that is impacted.
https://www.nbcnews.com/news/amp/rcna104424", 4, null);



insert into posts(id, content, parent_id, question_id) values (6, "Right now, real wages for the average American worker is higher than it was before the pandemic, with lower wage workers seeing the largest gains.

  That's Bidenomics.", null, null);
insert into posts(id, content, parent_id, question_id) values (7, "The tweet’s claim about real wages contains a factual error. On 3/15/20 when US COVID lockdowns began real wages adjusted for inflation (AFI) were $11.15. As of 7/16/23 real wages AFI are $11.05. Real wages AFI remain lower (not higher) than before the pandemic.", 6, null);




insert into users(id, secret) values (100, "secret100");
insert into vote_history(user_id, post_id, note_id, direction) values (100, 1, null, 1);
insert into vote_history(user_id, post_id, note_id, direction) values (100, 2, 3, 1);  // agreed with 2 (shown 3)
insert into vote_history(user_id, post_id, note_id, direction) values (100, 1, 2, -1); // changed mind after seeing 2
insert into vote_history(user_id, post_id, note_id, direction) values (100, 1, 2, 1); // changed mind back (for no reason)
insert into vote_history(user_id, post_id, note_id, direction) values (100, 1, 2, -1); // changed mind again (for no reason)



insert into users(id, secret) values (101, "secret101");
insert into vote_history(user_id, post_id, note_id, direction) values (101, 1, 2, -1);

insert into vote_history(user_id, post_id, note_id, direction) values (101, 1, 3, -1);
insert into vote_history(user_id, post_id, note_id, direction) values (101, 1, 3, 1);


