use anyhow::Result;
use actix_web::{web, guard, Responder, Error};
use actix_web_lab::respond::Html;

use crate::db::Pool;
use crate::models::Note;
use super::handlers::notes;

/// Actix configuration helper
pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/notes")
      .service(
        web::resource("")
          .route(
            web::route()
              .guard(guard::Header("accept", "application/json"))
              .to(notes::index_json)
          )
          .route(web::get().to(notes::index_html))
      )
      .service(
        web::scope("/{id}")
          .service(
            web::resource("")
              .route(
                web::route()
                  .guard(guard::Header("accept", "application/json"))
                  .to(notes::show_json)
              )
              .route(web::get().to(notes::show_html))

          )
      )
  );
}

/// Note update HTML page
async fn update(tmpl: web::Data<tera::Tera>, pool: web::Data<Pool>, path: web::Path<i32>) -> Result<impl Responder, Error> {
  let id = path.into_inner();

  let note = web::block(move || {
    let mut conn = pool.get()?;
    Note::find_by_id(id, &mut conn)
  })
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


