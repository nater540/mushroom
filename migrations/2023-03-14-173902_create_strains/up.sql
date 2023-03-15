CREATE TABLE strains (
  id INTEGER NOT NULL PRIMARY KEY,
  name VARCHAR NOT NULL,
  slug VARCHAR NOT NULL,
  description VARCHAR,
  species_id INTEGER NOT NULL
);

CREATE UNIQUE INDEX idx_strains_species_id ON strains(species_id);
CREATE UNIQUE INDEX idx_strains_slug ON strains(slug);
