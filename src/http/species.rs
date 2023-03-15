use anyhow::Result;
use actix_web::{web, HttpResponse, Responder, Error};
use actix_web_lab::respond::Html;

use crate::db::Pool;
use crate::models::{Mushroom, Species};

/// Actix configuration helper
pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::resource("/species")
      .route(web::get().to(index))
      .route(web::head().to(HttpResponse::MethodNotAllowed))
  );

  cfg.service(
    web::resource("/species/{slug}")
      .route(web::get().to(show))
      .route(web::head().to(HttpResponse::MethodNotAllowed))
  );
}

/// Index listing page
async fn index(tmpl: web::Data<tera::Tera>, pool: web::Data<Pool>) -> Result<impl Responder, Error> {
  // Fetch all species
  let species = web::block(move || {
    let mut conn = pool.get()?;
    Species::all(&mut conn)
  })
  .await?
  .map_err(actix_web::error::ErrorInternalServerError)?;

  let mut ctx = tera::Context::new();
  ctx.insert("title", "Species");
  ctx.insert("species", &species);

  Ok(Html(
    tmpl.render("species/index.html", &ctx)
    .map_err(|err| { dbg!(err); actix_web::error::ErrorInternalServerError("Template error") })?
  ))
}

/// Individual species view page
async fn show(tmpl: web::Data<tera::Tera>, pool: web::Data<Pool>, path: web::Path<String>) -> Result<impl Responder, Error> {
  let slug = path.into_inner();

  let (species, mushrooms) = web::block(move || show_query(slug, pool))
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

  let mut ctx = tera::Context::new();

  if let Some(species) = species {
    ctx.insert("title", &species.name);
    ctx.insert("species", &species);
    ctx.insert("mushrooms", &mushrooms);

    Ok(Html(
      tmpl.render("species/show.html", &ctx)
      .map_err(|err| { dbg!(err); actix_web::error::ErrorInternalServerError("Template error") })?
    ))
  }
  else {
    // Strain wasn't found - TODO: clean this up
    Ok(Html(
      tmpl.render("error.html", &ctx)
      .map_err(|err| { dbg!(err); actix_web::error::ErrorInternalServerError("Template error") })?
    ))
  }
}

fn show_query(slug: String, pool: web::Data<Pool>) -> Result<(Option<Species>, Option<Vec<Mushroom>>)> {
  let mut conn = pool.get()?;

  let species = Species::find_by_slug(slug, &mut conn)?;

  if let Some(species) = species {
    let mushrooms = Mushroom::find_by_species_id(species.id, &mut conn)?;
    Ok(
      (Some(species), Some(mushrooms))
    )
  }
  else {
   Ok((None, None))
  }
}
