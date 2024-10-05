-- Your SQL goes here

CREATE TABLE IF NOT EXISTS public."users" (
    id_users UUID NOT NULL,
    --id_users UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    first_name VARCHAR,
    last_name VARCHAR,
    email VARCHAR,
    status BOOLEAN NOT NULL DEFAULT FALSE,
    CONSTRAINT "users" PRIMARY KEY (id_users)
)
