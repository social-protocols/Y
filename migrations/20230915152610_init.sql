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
      , direction
      , max(created) AS created
    FROM vote_history
    GROUP BY 1,2
) select * from latest where direction != 0;


-- current_tally counts takes the latest vote for each user, regardless of whether it is informed or not.
create view stats as
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




drop view if exists informed_tally;
create view informed_tally as
with informed_votes as (
    SELECT
      user_id
      , post_id
      , note_id
      , direction
      , max(created) AS created
    FROM vote_history
    WHERE direction != 0
    and note_id is not null
    GROUP BY 1,2,3
)
select
  A.post_id
  , A.note_id

  , count(*)                                            as votes_given_seen_note

  , sum(case when A.direction = 1 then 1 else 0 end) as upvotes_given_seen_note

  , sum(case when A.direction = 1 
              and B.direction = 1 then 1 else 0 end) as upvotes_given_upvoted_note

  , sum(case when B.direction = 1 then 1 else 0 end) as votes_given_upvoted_note

  , sum(case when A.direction = 1 
              and B.direction = -1 then 1 else 0 end) as upvotes_given_downvoted_note

  , sum(case when B.direction = -1 then 1 else 0 end) as votes_given_downvoted_note
from 
    informed_votes A
    left join current_vote B
    on (A.note_id = B.post_id and A.user_id = B.user_id)
group by 1,2;



insert into posts(id, content, parent_id, question_id) values (1, "So, pregnant people can’t cross state lines to get abortions but guys like Kyle Rittenhouse can cross state lines to murder people. Seems fair.", null, null);
insert into posts(id, content, parent_id, question_id) values (2, "Kyle Rittenhouse was acquitted of murder charges. Clear video evidence showed he acted in self defense.", 1, null);
insert into posts(id, content, parent_id, question_id) values (3, "That trial was a sham. All the jurors where white.", 2, null);



insert into users(id, secret) values (100, "secret100");
insert into vote_history(user_id, post_id, note_id, direction) values (100, 1, null, 1);
insert into vote_history(user_id, post_id, note_id, direction) values (100, 1, 2, -1);

insert into users(id, secret) values (101, "secret101");
insert into vote_history(user_id, post_id, note_id, direction) values (101, 1, 2, -1);

