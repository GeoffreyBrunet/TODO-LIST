-- Your SQL goes here
CREATE TABLE todolist(
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  description VARCHAR NOT NULL,
  deadline VARCHAR NOT NULL,
  done BOOLEAN NOT NULL
)

INSERT INTO todolist (title, description, deadline, done)
  VALUES
    ("First task", "Need to do this", "Demain", FALSE)