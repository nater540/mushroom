CREATE TABLE mushroom_notes (
  id INTEGER NOT NULL PRIMARY KEY,
  mushroom_id INTEGER NOT NULL,
  title VARCHAR NOT NULL,
  body VARCHAR NOT NULL
);

CREATE INDEX idx_mushroom_id ON mushroom_notes(mushroom_id);
