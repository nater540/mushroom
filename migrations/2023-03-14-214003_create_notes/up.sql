CREATE TABLE notes (
  id INTEGER NOT NULL PRIMARY KEY,
  mushroom_id INTEGER NOT NULL,
  title VARCHAR NOT NULL,
  body VARCHAR NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY(mushroom_id) REFERENCES mushrooms(id)
);

CREATE INDEX idx_notes_mushroom_id ON notes(mushroom_id);
