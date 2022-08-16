mod routes;
pub mod schema;

use rocket::{build, launch, Build, Rocket};
use rocket_db_pools::sqlx::SqlitePool;
use rocket_db_pools::Database;
use routes::{catchers, routes};

#[derive(Database)]
#[database("wiki")]
pub struct WikiDatabase(SqlitePool);

#[launch]
fn rocket() -> Rocket<Build> {
  build()
    .register("/", catchers())
    .mount("/", routes())
    .attach(WikiDatabase::init())
}
