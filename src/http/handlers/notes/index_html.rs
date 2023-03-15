use anyhow::Result;
use actix_web::{web, guard, http::header, HttpResponse, Responder, Error};
use actix_web_lab::respond::Html;

use crate::db::Pool;
use crate::models::Note;

pub async fn index_html(tmpl: web::Data<tera::Tera>, pool: web::Data<Pool>) -> Result<impl Responder, Error> {
  let notes = web::block(move || super::index_query(pool))
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

  let mut ctx = tera::Context::new();
  ctx.insert("title", "Notes");
  ctx.insert("notes", &notes);

  Ok(Html(
    tmpl.render("notes/index.html", &ctx)
    .map_err(|err| { dbg!(err); actix_web::error::ErrorInternalServerError("Template error") })?
  ))
}
