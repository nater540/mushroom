use actix_web::{web, Responder, Error};
use actix_web_lab::respond::Html;

use crate::db::Pool;

pub async fn show_html(tmpl: web::Data<tera::Tera>, pool: web::Data<Pool>, path: web::Path<i32>) -> anyhow::Result<impl Responder, Error> {
  let id = path.into_inner();

  let note = web::block(move || super::note_by_id_query(id, pool))
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

  let mut ctx = tera::Context::new();

  if let Some(note) = note {
    ctx.insert("title", &note.title);
    ctx.insert("note", &note);

    Ok(Html(
      tmpl.render("notes/show.html", &ctx)
      .map_err(|err| { dbg!(err); actix_web::error::ErrorInternalServerError("Template error") })?
    ))
  }
  else {
    // Note wasn't found - TODO: clean this up
    Ok(Html(
      tmpl.render("error.html", &ctx)
      .map_err(|err| { dbg!(err); actix_web::error::ErrorInternalServerError("Template error") })?
    ))
  }
}
