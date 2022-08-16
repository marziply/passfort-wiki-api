mod routes;
pub mod schema;

use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::{build, launch, Build, Response, Rocket};
use rocket_db_pools::sqlx::SqlitePool;
use rocket_db_pools::Database;
use routes::{catchers, routes};
use schema::DocumentWithRevisions as DocRev;
use serde_json::from_str;

#[derive(Database)]
#[database("wiki")]
pub struct WikiDatabase(SqlitePool);

// A workaround for the NotFound issues
// Probably not the best way to do it though
async fn actually_not_found(res: &mut Response<'_>) {
  let body = res.body_mut();
  let text = body
    .to_string()
    .await
    .unwrap_or_default();

  if let Ok(doc) = from_str::<DocRev>(text.as_str()) {
    if doc.document.id.is_empty() {
      res.set_status(Status::NotFound);
    }
  }
}

#[launch]
fn rocket() -> Rocket<Build> {
  build()
    .register("/", catchers())
    .mount("/", routes())
    .attach(WikiDatabase::init())
    .attach(AdHoc::on_response("not-found", |_, res| {
      Box::pin(actually_not_found(res))
    }))
}
