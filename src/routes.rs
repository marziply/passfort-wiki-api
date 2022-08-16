use crate::schema::{Document, DocumentWithRevisions, Revision};
use crate::WikiDatabase;
use chrono::ParseError;
use rocket::http::Status;
use rocket::request::FromParam;
use rocket::serde::json::Json;
use rocket::{catch, catchers, get, post, routes, Catcher, Request, Route};
use rocket_db_pools::Connection;
use sqlx::query_as;
use sqlx::types::chrono::{DateTime, Utc};

type DB = Connection<WikiDatabase>;

#[derive(Debug)]
struct Timestamp(pub DateTime<Utc>);

impl<'r> FromParam<'r> for Timestamp {
  type Error = ParseError;

  fn from_param(param: &'r str) -> Result<Timestamp, Self::Error> {
    let date = DateTime::parse_from_rfc3339(param)?;

    Ok(Timestamp(DateTime::from(date)))
  }
}

async fn fetch_document(db: &mut DB, identifier: String) -> Document {
  let title = format!("{}", identifier.replace("-", " "));

  query_as!(
    Document,
    r#"
      SELECT
        *
      FROM
        documents
      WHERE
        title = ?
        OR id = ?
    "#,
    title,
    identifier
  )
  .fetch_one(db.as_mut())
  .await
  .unwrap_or_default()
}

#[get("/documents")]
async fn list_documents(
  mut db: Connection<WikiDatabase>,
) -> Json<Vec<Document>> {
  query_as!(Document, "SELECT * FROM documents")
    .fetch_all(&mut *db)
    .await
    .map_or_else(|_| Json(Vec::new()), |v| Json(v))
}

#[get("/documents/<identifier>")]
async fn get_document(
  mut db: DB,
  identifier: String,
) -> Json<DocumentWithRevisions> {
  let document = fetch_document(&mut db, identifier).await;
  let revisions = query_as!(
    Revision,
    r#"
      SELECT
        *
      FROM
        revisions
      WHERE
        document_id = ?
    "#,
    document.id
  )
  .fetch_all(&mut *db)
  .await
  .unwrap_or_default();

  Json(DocumentWithRevisions {
    document,
    revisions,
  })
}

#[get("/documents/<identifier>/<timestamp>")]
async fn get_document_at(
  mut db: DB,
  identifier: String,
  timestamp: Timestamp,
) -> Json<Revision> {
  let document = fetch_document(&mut db, identifier).await;
  let date = timestamp.0.to_rfc3339();

  query_as!(
    Revision,
    r#"
      SELECT
        *
      FROM
        revisions
      WHERE
        document_id = ?
        AND created_at > ?
      ORDER BY
        created_at ASC
    "#,
    document.id,
    date
  )
  .fetch_one(&mut *db)
  .await
  .map_or_else(|_| Json(Revision::default()), |v| Json(v))
}

#[post("/documents/<title>")]
async fn new_document(title: String) {}

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
