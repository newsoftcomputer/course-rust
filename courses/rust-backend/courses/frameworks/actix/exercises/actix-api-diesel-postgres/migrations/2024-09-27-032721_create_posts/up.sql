-- Your SQL goes here


CREATE TABLE public."users" (
  id_users PRIMARY KEY NOT NULL,
  firstname VARCHAR,
  lastname VARCHAR,
  email VARCHAR,
  status BOOLEAN NOT NULL DEFAULT FALSE
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public."users"
  OWNER to andresgiraldo;


CREATE TABLE public."posts" (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT FALSE
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public."posts"
    OWNER to andresgiraldo;
