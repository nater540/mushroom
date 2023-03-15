mod dashboard;
mod species;
mod notes;
mod handlers;

use actix_web::{
  body::BoxBody,
  dev::ServiceResponse,
  http::{header::ContentType, StatusCode},
  middleware::{self, ErrorHandlerResponse, ErrorHandlers},
  web, App, HttpResponse, HttpServer
};
use tera::Tera;

use crate::db::Pool;

pub async fn start_server(connection: Pool) -> anyhow::Result<()> {
  HttpServer::new(move || {
    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

    App::new()
      .app_data(web::Data::new(tera))
      .app_data(web::Data::new(connection.clone()))
      .wrap(middleware::Logger::default())
      .configure(dashboard::config)
      .configure(species::config)
      .configure(notes::config)
      .service(actix_files::Files::new("/assets", "./assets/").show_files_listing())
      .service(web::scope("").wrap(error_handlers()))
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await?;

  Ok(())
}

fn error_handlers() -> ErrorHandlers<BoxBody> {
  ErrorHandlers::new().handler(StatusCode::NOT_FOUND, not_found)
}

fn not_found<B>(res: ServiceResponse<B>) -> actix_web::Result<ErrorHandlerResponse<BoxBody>> {
  let response = get_error_response(&res, "Page not found");
  Ok(ErrorHandlerResponse::Response(ServiceResponse::new(
    res.into_parts().0,
    response.map_into_left_body()
  )))
}

fn get_error_response<B>(res: &ServiceResponse<B>, error: &str) -> HttpResponse {
  let request = res.request();

  let fallback = |err: &str| {
    HttpResponse::build(res.status())
      .content_type(ContentType::plaintext())
      .body(err.to_string())
  };

  let tera = request.app_data::<web::Data<Tera>>().map(|t| t.get_ref());
  match tera {
    Some(tera) => {
      let mut context = tera::Context::new();
      context.insert("error", error);
      context.insert("status_code", res.status().as_str());
      let body = tera.render("error.html", &context);

      match body {
        Ok(body) => HttpResponse::build(res.status())
          .content_type(ContentType::html())
          .body(body),
        Err(_) => fallback(error)
      }
    }
    None => fallback(error)
  }
}
