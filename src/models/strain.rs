use anyhow::Result;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db::schema::strains;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Strain {
  pub id: i32,
  pub name: String,
  pub slug: String,
  pub description: Option<String>,
  pub species_id: i32
}

impl Strain {
  pub fn all(conn: &mut SqliteConnection) -> Result<Vec<Strain>> {
    Ok(
      strains::table
        .order(strains::name.asc())
        .limit(500)
        .load::<Strain>(conn)?
    )
  }

  pub fn find_by_species_id(species_id: i32, conn: &mut SqliteConnection) -> Result<Vec<Strain>> {
    Ok(
      strains::table
        .filter(strains::species_id.eq(species_id))
        .order(strains::name.asc())
        .limit(500)
        .load::<Strain>(conn)?
    )
  }

  pub fn find_by_slug(slug: String, conn: &mut SqliteConnection) -> Result<Option<Strain>> {
    Ok(
      strains::table
        .filter(strains::slug.eq(slug))
        .first::<Strain>(conn)
        .optional()?
    )
  }

  pub fn find_by_id(id: i32, conn: &mut SqliteConnection) -> Result<Option<Strain>> {
    Ok(
      strains::table
        .filter(strains::id.eq(id))
        .first::<Strain>(conn)
        .optional()?
    )
  }
}
