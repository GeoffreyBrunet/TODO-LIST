-- Your SQL goes here
CREATE TABLE todolist(
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  description VARCHAR NOT NULL,
  deadline DATE NOT NULL,
  done INT NOT NULL
)