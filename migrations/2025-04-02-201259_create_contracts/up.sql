-- Your SQL goes here
CREATE TABLE users(
    id SERIAL PRIMARY KEY,
    name varchar(128) NOT NULL,
    email varchar(128) NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMP DEFAULT NOW() NOT NULL
);

CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    author integer NOT NULL REFERENCES users(id),
    comment text NOT NULL DEFAULT '',
    created_at TIMESTAMP DEFAULT NOW() NOT NULL
);

CREATE TABLE collaborators(
    id SERIAL PRIMARY KEY,
    user_id integer NOT NUll REFERENCES users(id),
    project_id integer NOT NULL REFERENCES projects(id)
);


CREATE TABLE contracts (
    id SERIAL PRIMARY KEY,
    project_id integer NOT NULL REFERENCES projects(id),
    author_id integer NOT NULL REFERENCES users(id),
    comment_id integer NOT NULL REFERENCES comments(id),
    grpc_method varchar(128) NOT NULL,
    tag varchar(128) DEFAULT '',
    errors_response varchar(128),

    path varchar(256),
    query varchar(128),
    body varchar(256),
    response text,

    created_at TIMESTAMP DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMP DEFAULT NOW() NOT NULL
);

ALTER TABLE projects DROP COLUMN IF EXISTS creator_id;