CREATE TABLE mushrooms (
  id INTEGER NOT NULL PRIMARY KEY,
  parent_id INTEGER DEFAULT NULL,
  name VARCHAR NOT NULL,
  label VARCHAR DEFAULT NULL,
  species_id INTEGER NOT NULL,
  strain_id INTEGER NOT NULL,
  petri_dish BOOLEAN NOT NULL DEFAULT TRUE,
  germination_date TIMESTAMP DEFAULT NULL,
  FOREIGN KEY(species_id) REFERENCES species(id),
  FOREIGN KEY(strain_id) REFERENCES strains(id)
);

CREATE INDEX idx_mushrooms_parent_id ON mushrooms(parent_id);
CREATE INDEX idx_mushrooms_species_id ON mushrooms(species_id);
CREATE INDEX idx_mushrooms_strain_id ON mushrooms(strain_id);
