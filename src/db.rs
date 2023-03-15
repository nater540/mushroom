use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;

pub mod schema;

pub type Pool = diesel::r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn establish_connection() -> anyhow::Result<Pool> {
  let manager = ConnectionManager::<SqliteConnection>::new("sqlite://mushrooms.db");
  Ok(
    diesel::r2d2::Pool::builder()
      .test_on_check_out(true)
      .build(manager)?
  )
}
