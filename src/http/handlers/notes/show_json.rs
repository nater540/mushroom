use actix_web::{web, Responder, Error};

use crate::db::Pool;

pub async fn show_json(pool: web::Data<Pool>, path: web::Path<i32>) -> anyhow::Result<impl Responder, Error> {
  let id = path.into_inner();

  let note = web::block(move || super::note_by_id_query(id, pool))
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

  Ok(web::Json(note))
}
