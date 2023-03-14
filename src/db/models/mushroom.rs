use anyhow::Result;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db::schema::mushrooms;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Mushroom {
  pub id: i32,
  pub parent_id: Option<i32>,
  pub name: String,
  pub label: Option<String>,
  pub species_id: i32,
  pub strain_id: i32,
  pub petri_dish: bool,
  pub germination_date: Option<NaiveDateTime>
}

impl Mushroom {
  pub fn all(conn: &mut SqliteConnection) -> Result<Vec<Mushroom>> {
    Ok(
      mushrooms::table
        .order(mushrooms::germination_date.desc())
        .limit(500)
        .load::<Mushroom>(conn)?
    )
  }

  pub fn find_by_parent_id(parent_id: i32, conn: &mut SqliteConnection) -> Result<Vec<Mushroom>> {
    Ok(
      mushrooms::table
        .filter(mushrooms::parent_id.eq(parent_id))
        .order(mushrooms::germination_date.desc())
        .limit(500)
        .load::<Mushroom>(conn)?
    )
  }

  pub fn find_by_species_id(species_id: i32, conn: &mut SqliteConnection) -> Result<Vec<Mushroom>> {
    Ok(
      mushrooms::table
        .filter(mushrooms::species_id.eq(species_id))
        .order(mushrooms::germination_date.desc())
        .limit(500)
        .load::<Mushroom>(conn)?
    )
  }

  pub fn find_by_id(id: i32, conn: &mut SqliteConnection) -> Result<Option<Mushroom>> {
    Ok(
      mushrooms::table
        .filter(mushrooms::id.eq(id))
        .first::<Mushroom>(conn)
        .optional()?
    )
  }
}
