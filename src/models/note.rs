use anyhow::Result;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::db::schema::notes;

#[derive(Default, Debug, Clone, Serialize, Deserialize, Queryable, Associations)]
#[diesel(belongs_to(super::Mushroom))]
pub struct Note {
  pub id: i32,
  pub mushroom_id: i32,
  pub title: String,
  pub body: String,
  pub created_at: Option<NaiveDateTime>
}

impl Note {
  pub fn all(conn: &mut SqliteConnection) -> Result<Vec<Note>> {
    Ok(
      notes::table
        .order(notes::created_at.asc())
        .limit(500)
        .load::<Note>(conn)?
    )
  }

  pub fn find_by_id(id: i32, conn: &mut SqliteConnection) -> Result<Option<Note>> {
    Ok(
      notes::table
        .filter(notes::id.eq(id))
        .first::<Note>(conn)
        .optional()?
    )
  }

  // pub fn find_by_mushroom_id(mushroom_id: i32, conn: &mut SqliteConnection) -> Result<Vec<Note>> {
  //   Ok(
  //     notes::table
  //       .filter(notes::mushroom_id.eq(mushroom_id))
  //       .order(notes::created_at.asc())
  //       .limit(500)
  //       .load::<Note>(conn)?
  //   )
  // }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = notes)]
pub struct NewNote<'a> {
  pub mushroom_id: i32,
  pub title: Cow<'a, str>,
  pub body: Cow<'a, str>
}

impl<'a> NewNote<'a> {
  pub fn create<S>(mushroom_id: i32, title: S, body: S, conn: &mut SqliteConnection) -> Result<usize>
    where S: Into<Cow<'a, str>> {

    let title = title.into();
    let body = body.into();
    let item = NewNote { mushroom_id, title, body };
    item.insert(conn)
  }

  fn insert(&self, conn: &mut SqliteConnection) -> Result<usize> {
    Ok(
      diesel::insert_into(notes::table)
        .values(self)
        .execute(conn)?
    )
  }
}
