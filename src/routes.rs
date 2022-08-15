use crate::schema::Document;
use chrono::{DateTime, FixedOffset, ParseError};
use rocket::request::FromParam;
use rocket::serde::json::Json;
use rocket::{catch, catchers, get, post, routes, Catcher, Route};

struct Timestamp(DateTime<FixedOffset>);

impl<'r> FromParam<'r> for Timestamp {
  type Error = ParseError;

  fn from_param(param: &'r str) -> Result<Timestamp, Self::Error> {
    Ok(Timestamp(DateTime::parse_from_rfc3339(param)?))
  }
}

#[get("/documents")]
fn list_documents() -> Json<Vec<Document>> {
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

pub(super) fn routes() -> Vec<Route> {
  routes![list_documents, get_document, get_document_at, new_document]
}

pub(super) fn catchers() -> Vec<Catcher> {
  catchers![not_found]
}
