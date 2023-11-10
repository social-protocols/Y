CREATE TABLE users (
    id      integer   not null primary key -- rowid
  , secret  text      not null unique
  , created TIMESTAMP not null DEFAULT CURRENT_TIMESTAMP
);

-- if a question is a top-level question -> no parent and question_id = id
-- if a question is a reply -> parent is the id of the post it replies to
-- -- if it reuses a question -> that question is its question_id
-- -- if it doesn't reuse a question -> question_id = id
create table posts (
      id          integer   primary key -- row id
    , parent_id   integer   references posts (id)
    , content     text      not null
    , question_id integer   references posts (id)
    , author_id   integer   not null references users (id)
    , created     TIMESTAMP not null DEFAULT CURRENT_TIMESTAMP
);

create table vote_history (
      user_id   not null references users (id)
    , tag_id    not null references tags (id) -- TODO rename
    , post_id   not null references posts (id)
    , note_id   references posts (id)
    , direction integer not null
    , created   TIMESTAMP not null DEFAULT CURRENT_TIMESTAMP
);



-- The current (latest) votes for all users on all posts
-- If the user has cleared their vote, no row is returned.
CREATE VIEW current_vote as
with latest as (
    SELECT
        user_id
      , tag_id
      , post_id

      -- NOTE: direction will be the value of direction pulled from the same row that has max(created)
      -- https://www.sqlite.org/lang_select.html#bareagg
      , direction
      , max(created) AS created
    FROM vote_history
    GROUP BY user_id, post_id, tag_id
)
-- The latest vote might be zero, so in that case we don't return a record for this user and post
select *
from latest
where direction != 0;


-- current_tally counts the latest votes, regardless of whether they are informed or not.
create view current_tally as
select
    tag_id
  , post_id
  , sum(
      case direction
        when 1 then 1
        else 0
      end
    ) as upvotes
  , count(*) as votes
  -- , sum(case when note_id is null and direction = 1 then 1 else 0 end) as upvotes_given_not_seen_any_note
  -- , sum(case when note_id is null then 1 else 0 end) as votes_given_not_seen_any_note
from current_vote
group by tag_id, post_id;

drop view if exists current_informed_tally;
create view current_informed_tally as
with current_informed_votes as (
    SELECT
        user_id
      , tag_id
      , post_id
      , note_id

      -- NOTE: direction will be the value of direction pulled from the same row that has max(created)
      -- https://www.sqlite.org/lang_select.html#bareagg
      , direction
      , max(created) AS created
    FROM vote_history
    where note_id is not null
    GROUP BY 
        user_id
      , tag_id
      , post_id
      , note_id
)
, informed_tally as (
  select 
      tag_id
    , post_id
    , note_id
    , sum(
      case
        when direction = 1 then 1
        else 0
      end
    ) as upvotes
    , count(*) as votes
  from current_informed_votes
  -- The latest vote might be zero, so in that case we don't return a record for this user and post
  where direction != 0
  group by tag_id, post_id, note_id
),  
first_votes_on_notes as (
  SELECT 
        user_id
        , tag_id
        , post_id
        , note_id
        -- , direction
        , min(rowid) first_vote_on_this_note_rowid
  FROM vote_history
  WHERE note_id is not null
  GROUP BY user_id, tag_id, post_id, note_id
)
, votes_before_note as (
    select
      params.tag_id as p_tag_id
      , params.post_id as p_post_id
      , params.note_id as p_note_id
      -- , first_votes_on_notes.tag_id as f_tag_id
      -- , first_votes_on_notes.post_id as f_post_id
      -- , first_votes_on_notes.note_id as f_note_id
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
       join vote_history using (tag_id, post_id)
    LEFT OUTER JOIN first_votes_on_notes on (
           first_votes_on_notes.tag_id = params.tag_id
       and first_votes_on_notes.post_id = params.post_id
       and first_votes_on_notes.note_id = params.note_id
       and first_votes_on_notes.user_id = vote_history.user_id
    )
)
, last_votes_before_note as (
    select
        p_tag_id as tag_id
        , p_post_id as post_id
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
    group by p_tag_id, p_post_id, p_note_id, user_id
)
select
    tag_id
  , post_id
  , note_id
  , sum(
    case direction
      when 1 then 1
      else 0
    end 
  ) as upvotes_given_not_shown_this_note
  , count(*) as votes_given_not_shown_this_note

  , upvotes_given_shown_this_note
  , votes_given_shown_this_note
from last_votes_before_note
group by tag_id, post_id, note_id;



-- drop view if exists probabilities_given_note;
-- create view probabilities_given_note as
-- with parameters as (
--     select
--           .85 as prior
--         , 2 as priorWeight
-- )
-- , given_not_shown_this_note as (
--     select 
--         *
--         , (cast(upvotes_given_not_shown_this_note + prior*priorWeight as float)) / (cast(votes_given_not_shown_this_note + priorWeight as float)) as p_given_not_shown_this_note
--     from current_informed_tally join parameters
-- )
-- , given_shown_this_note as (
--     select 
--         post_id
--         , note_id 
--         , upvotes_given_not_shown_this_note 
--         , votes_given_not_shown_this_note 
--         , p_given_not_shown_this_note 
--         , upvotes_given_shown_this_note 
--         , votes_given_shown_this_note  
--         , ( upvotes_given_shown_this_note + p_given_not_shown_this_note * priorWeight) / (cast(votes_given_shown_this_note + priorWeight as float)) as p_given_shown_this_note
--     from given_not_shown_this_note
-- )
-- select 
--     * 
--     -- , max("p(A=1|vA,sB)")
--     from given_shown_this_note
--     -- group by post_id
-- ;


create table tags (
    id integer not null primary key
  , tag text not null
  , unique (tag)
);

-- post_tag
-- create table tags (
--     post_id integer not null references posts (id) on delete cascade on update cascade
--   , tag     text    not null
--   , unique(post_id, tag)
-- );


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



insert into users(id, secret) values (100, "secret100");
insert into users(id, secret) values (101, "secret101");

insert into tags(id, tag) values (0, 'global');


insert into posts(id, parent_id, author_id, content) values (1, null, 100, 'So, pregnant people can’t cross state lines to get abortions but guys like Kyle Rittenhouse can cross state lines to murder people. Seems fair.');
insert into vote_history(tag_id, post_id, note_id, user_id, direction) values (0, 1, null, 100, 1);
insert into posts(id, parent_id, author_id, content) values (2, 1, 101, 'Kyle Rittenhouse was acquitted of murder charges. Clear video evidence showed he acted in self defense.');
insert into vote_history(tag_id, post_id, note_id, user_id, direction) values (0, 2, null, 101, 1);
insert into posts(id, parent_id, author_id, content) values (3, 2, 100, 'That trial was a sham. They were never going to convict.');
insert into vote_history(tag_id, post_id, note_id, user_id, direction) values (0, 3, null, 100, 1);


insert into posts(id, parent_id, author_id, content) values (4, null, 100, 'Sudafed, Benadryl and most decongestants don’t work: FDA advisory panel https://trib.al/sJmOJBP');
insert into vote_history(tag_id, post_id, note_id, user_id, direction) values (0, 4, null, 100, 1);
insert into posts(id, parent_id, author_id, content) values (5, 4, 100, 'This is misleading. Regular Benadryl is an antihistamine; it is not a decongestant. There is a Benadryl branded product that is impacted. https://www.nbcnews.com/news/amp/rcna104424');
insert into vote_history(tag_id, post_id, note_id, user_id, direction) values (0, 5, null, 101, 1);

insert into posts(id, parent_id, author_id, content) values (6, null, 100, 'Right now, real wages for the average American worker is higher than it was before the pandemic, with lower wage workers seeing the largest gains. That''s Bidenomics.');
insert into vote_history(tag_id, post_id, note_id, user_id, direction) values (0, 6, null, 100, 1);
insert into posts(id, parent_id, author_id, content) values (7, 6, 100, 'The tweet’s claim about real wages contains a factual error. On 3/15/20 when US COVID lockdowns began real wages adjusted for inflation (AFI) were $11.15. As of 7/16/23 real wages AFI are $11.05. Real wages AFI remain lower (not higher) than before the pandemic.');
insert into vote_history(tag_id, post_id, note_id, user_id, direction) values (0, 7, null, 101, 1);




insert into vote_history(tag_id, post_id, note_id, user_id, direction) values (0, 2, 3, 100, 1);  --agreed with 2 (shown 3)
insert into vote_history(tag_id, post_id, note_id, user_id, direction) values (0, 1, 2, 100, -1); --changed mind after seeing 2
insert into vote_history(tag_id, post_id, note_id, user_id, direction) values (0, 1, 2, 100, 1);  --changed mind back (for no reason)
insert into vote_history(tag_id, post_id, note_id, user_id, direction) values (0, 1, 2, 100, -1); --changed mind again (for no reason)



insert into vote_history(tag_id, post_id, note_id, user_id, direction) values (0, 1, 2, 101, -1);

insert into vote_history(tag_id, post_id, note_id, user_id, direction) values (0, 1, 3, 101, -1);
insert into vote_history(tag_id, post_id, note_id, user_id, direction) values (0, 1, 3, 101, 1);


