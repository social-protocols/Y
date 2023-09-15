CREATE TABLE posts (
    id integer primary key -- row id
    , content text not null
    , parent integer references posts (id) on delete cascade on update cascade -- nullable
    , question_id integer references posts (id) on delete cascade on update cascade -- nullable
        -- if a question is a top-level question -> no parent and question_id = id
        -- if a question is a reply -> parent is the id of the post it replies to
        -- -- if it reuses a question -> that question is its question_id
        -- -- if it doesn't reuse a question -> question_id = id
    , created TIMESTAMP not null DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE _sqlx_migrations (
    version BIGINT PRIMARY KEY,
    description TEXT NOT NULL,
    installed_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    success BOOLEAN NOT NULL,
    checksum BLOB NOT NULL,
    execution_time BIGINT NOT NULL
);
CREATE TABLE users (
  id integer not null primary key, -- rowid
  secret text not null unique,
  created TIMESTAMP not null DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE vote_history (
    post_id not null references posts (id) on delete cascade on update cascade
    , user_id not null references users (id) on delete cascade on update cascade
    , created TIMESTAMP not null DEFAULT CURRENT_TIMESTAMP
    , direction integer not null
);
