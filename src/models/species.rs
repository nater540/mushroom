use anyhow::Result;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db::schema::species;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Species {
  pub id: i32,
  pub name: String,
  pub slug: String,
  pub description: Option<String>,
  pub psillow_link: Option<String>
}

impl Species {
  pub fn all(conn: &mut SqliteConnection) -> Result<Vec<Species>> {
    Ok(
      species::table
        .order(species::name.asc())
        .limit(500)
        .load::<Species>(conn)?
    )
  }

  pub fn find_by_slug(slug: String, conn: &mut SqliteConnection) -> Result<Option<Species>> {
    Ok(
      species::table
        .filter(species::slug.eq(slug))
        .first::<Species>(conn)
        .optional()?
    )
  }
}
