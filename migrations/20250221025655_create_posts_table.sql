-- Add migration script here
CREATE TABLE posts (
   id TEXT PRIMARY KEY,
   title TEXT NOT NULL,
   content TEXT NOT NULL
);