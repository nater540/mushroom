CREATE TABLE species (
  id INTEGER NOT NULL PRIMARY KEY,
  name VARCHAR NOT NULL,
  slug VARCHAR NOT NULL,
  description VARCHAR,
  psillow_link VARCHAR
);

CREATE UNIQUE INDEX idx_species_slug ON species(slug);

INSERT INTO species(name, slug) VALUES("Psilocybe Liniformans", "liniformans");
INSERT INTO species(name, slug) VALUES("Psilocybe Yungensis", "yungensis");
INSERT INTO species(name, slug) VALUES("Psilocybe Mexicana", "mexicana");
INSERT INTO species(name, slug) VALUES("Psilocybe Makarorae", "makarorae");
INSERT INTO species(name, slug) VALUES("Psilocybe Caerulescens", "caerulescens");
INSERT INTO species(name, slug) VALUES("Psilocybe Allenii", "allenii");
INSERT INTO species(name, slug) VALUES("Psilocybe Subaeruginosa", "subaeruginosa");
INSERT INTO species(name, slug) VALUES("Psilocybe Subcaerulipes", "subcaerulipes");
INSERT INTO species(name, slug) VALUES("Psilocybe Quebecensis", "quebecensis");
INSERT INTO species(name, slug) VALUES("Psilocybe Strictipes", "strictipes");
INSERT INTO species(name, slug) VALUES("Psilocybe Stuntzii", "stuntzii");
INSERT INTO species(name, slug) VALUES("Psilocybe Galindoi", "galindoi");
INSERT INTO species(name, slug) VALUES("Psilocybe Baeocystis", "baeocystis");
INSERT INTO species(name, slug) VALUES("Psilocybe Ovoideocystidiata", "ovoideocystidiata");
INSERT INTO species(name, slug) VALUES("Psilocybe Zapotecorum", "zapotecorum");
INSERT INTO species(name, slug) VALUES("Psilocybe Aztecorum", "aztecorum");
INSERT INTO species(name, slug) VALUES("Psilocybe Pelliculosa", "pelliculosa");
INSERT INTO species(name, slug) VALUES("Psilocybe Caerulipes", "caerulipes");
INSERT INTO species(name, slug) VALUES("Psilocybe Weilii", "weilii");
INSERT INTO species(name, slug) VALUES("Psilocybe Cyanescens", "cyanescens");
INSERT INTO species(name, slug) VALUES("Psilocybe Semilanceata", "semilanceata");
INSERT INTO species(name, slug) VALUES("Psilocybe Subcubensis", "subcubensis");
INSERT INTO species(name, slug) VALUES("Psilocybe Samuiensis", "samuiensis");
INSERT INTO species(name, slug) VALUES("Psilocybe Aucklandii", "aucklandii");
INSERT INTO species(name, slug) VALUES("Psilocybe Fagocola", "fagocola");
INSERT INTO species(name, slug) VALUES("Psilocybe Serbica", "serbica");
INSERT INTO species(name, slug) VALUES("Psilocybe Cubensis", "cubensis");
INSERT INTO species(name, slug) VALUES("Psilocybe Azurescens", "azurescens");
