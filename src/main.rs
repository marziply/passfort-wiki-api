mod routes;

use rocket::{build, launch, Build, Rocket};
use routes::{catchers, routes};

#[launch]
fn rocket() -> Rocket<Build> {
  build()
    .register("/", catchers())
    .mount("/", routes())
}
