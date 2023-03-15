use anyhow::Result;
use actix_web::web;
use crate::db::Pool;
use crate::models::Note;

mod index_html;
mod index_json;
mod show_html;
mod show_json;

pub use index_html::index_html;
pub use index_json::index_json;
pub use show_html::show_html;
pub use show_json::show_json;

/// Returns all notes sorted by created_at ASC
fn index_query(pool: web::Data<Pool>) -> Result<Vec<Note>> {
  let mut conn = pool.get()?;
  Note::all(&mut conn)
}

/// Attempts to find a requested note
fn note_by_id_query(id: i32, pool: web::Data<Pool>) -> Result<Option<Note>> {
  let mut conn = pool.get()?;
  Note::find_by_id(id, &mut conn)
}
