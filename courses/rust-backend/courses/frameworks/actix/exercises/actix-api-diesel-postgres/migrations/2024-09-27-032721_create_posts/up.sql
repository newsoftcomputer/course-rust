-- Your SQL goes here


CREATE TABLE users (
    id_users SERIAL PRIMARY KEY,
    fisrt_name VARCHAR,
    lastname VARCHAR,
    email VARCHAR,
    status BOOLEAN NOT NULL DEFAULT FALSE
)

-- TABLESPACE pg_default;

ALTER TABLE IF EXISTS users
  OWNER to postgres;


CREATE TABLE posts (
  id_posts SERIAL PRIMARY KEY,
  title VARCHAR,
  body TEXT,
  published BOOLEAN NOT NULL DEFAULT FALSE
)

-- TABLESPACE pg_default;

ALTER TABLE IF EXISTS posts
    OWNER to postgres;
