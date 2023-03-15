#![allow(dead_code)]

use anyhow::Result;
use tracing_subscriber::FmtSubscriber;
use tracing::{info, Level};

mod db;
mod http;
mod models;

#[actix_web::main]
async fn main() -> Result<()> {
  init_logging()?;

  info!("Establishing database connection...");
  let connection = db::establish_connection()?;

  info!("Starting HTTP server at http://127.0.0.1:8080");
  http::start_server(connection).await?;

  Ok(())
}

fn init_logging() -> Result<()> {
  let subscriber = FmtSubscriber::builder()
    .with_max_level(Level::DEBUG)
    .finish();

  tracing::subscriber::set_global_default(subscriber)?;

  Ok(())
}
