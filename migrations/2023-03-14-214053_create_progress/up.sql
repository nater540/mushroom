CREATE TABLE progress (
  id INTEGER NOT NULL PRIMARY KEY,
  mushroom_id INTEGER NOT NULL,
  total REAL NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY(mushroom_id) REFERENCES mushrooms(id)
);

CREATE UNIQUE INDEX idx_progress_mushroom_id ON progress(mushroom_id);
