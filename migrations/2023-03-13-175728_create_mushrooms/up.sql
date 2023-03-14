CREATE TABLE mushrooms (
  id INTEGER NOT NULL PRIMARY KEY,
  parent_id INTEGER DEFAULT NULL,
  name VARCHAR NOT NULL,
  label VARCHAR DEFAULT NULL,
  species_id INTEGER NOT NULL,
  strain_id INTEGER NOT NULL,
  petri_dish BOOLEAN NOT NULL DEFAULT TRUE,
  germination_date TIMESTAMP DEFAULT NULL
);

CREATE INDEX idx_parent_id ON mushrooms(parent_id);
