use anyhow::Result;
use actix_web::{web, HttpResponse, Responder, Error};
use actix_web_lab::respond::Html;
use crate::db::Pool;
use crate::models::Mushroom;

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
      web::resource("/")
        .route(web::get().to(index))
        .route(web::head().to(HttpResponse::MethodNotAllowed))
  );
}

async fn index(tmpl: web::Data<tera::Tera>, pool: web::Data<Pool>) -> Result<impl Responder, Error> {
  // Fetch all mushrooms
  let mushrooms = web::block(move || {
    let mut conn = pool.get()?;
    Mushroom::all(&mut conn)
  })
  .await?
  .map_err(actix_web::error::ErrorInternalServerError)?;

  let mut ctx = tera::Context::new();
  ctx.insert("title", "Dashboard");
  ctx.insert("mushrooms", &mushrooms);

  Ok(Html(
    tmpl.render("dashboard.html", &ctx)
    .map_err(|err| { dbg!(err); actix_web::error::ErrorInternalServerError("Template error") })?
  ))
}
