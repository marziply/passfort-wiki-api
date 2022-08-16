use crate::schema::Document;
use crate::WikiDatabase;
use chrono::{DateTime, FixedOffset, ParseError};
use rocket::http::Status;
use rocket::request::FromParam;
use rocket::serde::json::Json;
use rocket::{catch, catchers, get, post, routes, Catcher, Request, Route};
use rocket_db_pools::sqlx::query;
use rocket_db_pools::Connection;

struct Timestamp(DateTime<FixedOffset>);

impl<'r> FromParam<'r> for Timestamp {
  type Error = ParseError;

  fn from_param(param: &'r str) -> Result<Timestamp, Self::Error> {
    Ok(Timestamp(DateTime::parse_from_rfc3339(param)?))
  }
}

#[get("/documents")]
async fn list_documents(
  mut db: Connection<WikiDatabase>,
) -> Json<Vec<Document>> {
  let documents = query("SELECT * FROM documents")
    .fetch_all(&mut *db)
    .await;

  Json(vec![])
}

#[get("/documents/<title>")]
fn get_document(title: String) {}

#[get("/documents/<title>/<timestamp>")]
fn get_document_at(title: String, timestamp: Timestamp) {}

#[post("/documents/<title>")]
fn new_document(title: String) {}

#[catch(404)]
fn not_found() -> &'static str {
  "not found"
}

#[catch(default)]
fn default_catch(status: Status, req: &Request) -> String {
  format!("{status} ({}): {}", req.uri(), status.reason_lossy())
}

pub(super) fn routes() -> Vec<Route> {
  routes![list_documents, get_document, get_document_at, new_document]
}

pub(super) fn catchers() -> Vec<Catcher> {
  catchers![not_found, default_catch]
}
