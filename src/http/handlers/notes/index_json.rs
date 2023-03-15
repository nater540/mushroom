use actix_web::{web, Responder, Error};

use crate::db::Pool;

pub async fn index_json(pool: web::Data<Pool>) -> anyhow::Result<impl Responder, Error> {
  let notes = web::block(move || super::index_query(pool))
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

  Ok(web::Json(notes))
}
