CREATE TABLE users (
  id integer not null primary key, -- rowid
  secret text not null unique,
  created TIMESTAMP not null DEFAULT CURRENT_TIMESTAMP
);
