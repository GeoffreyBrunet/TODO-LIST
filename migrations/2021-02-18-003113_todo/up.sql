-- Your SQL goes here
CREATE TABLE todo (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  description VARCHAR NOT NULL,
  deadline INTEGER NOT NULL,
  done BOOLEAN NOT NULL
)
