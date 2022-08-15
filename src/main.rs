mod routes;
pub mod schema;

use rocket::{build, launch, Build, Rocket};
use routes::{catchers, routes};

#[launch]
fn rocket() -> Rocket<Build> {
  build()
    .register("/", catchers())
    .mount("/", routes())
}
