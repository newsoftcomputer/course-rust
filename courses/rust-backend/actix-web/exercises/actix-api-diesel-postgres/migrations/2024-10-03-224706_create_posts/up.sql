-- Your SQL goes here


CREATE TABLE users (
    id_users uuid NOT NULL,
    first_name VARCHAR,
    last_name VARCHAR,
    email VARCHAR,
    status BOOLEAN NOT NULL DEFAULT FALSE
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS users
  OWNER to postgres;


CREATE TABLE posts (
  id_posts SERIAL NOT NULL ,
  title VARCHAR,
  body TEXT,
  published BOOLEAN NOT NULL DEFAULT FALSE
  CONSTRAINT "posts" PRIMARY KEY (id_posts)
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS posts
  OWNER to postgres;
