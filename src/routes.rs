use crate::schema::{Document, DocumentWithRevisions, NewDocument, Revision};
use crate::WikiDatabase;
use chrono::{DateTime, ParseError, Utc};
use rocket::form::Error as ValidationError;
use rocket::http::Status;
use rocket::request::FromParam;
use rocket::serde::json::Json;
use rocket::{catch, catchers, get, post, routes, Catcher, Request, Route};
use rocket_db_pools::Connection;
use serde::Deserialize;
use sqlx::query_as;
use std::str::FromStr;
use uuid::Uuid;

type DB = Connection<WikiDatabase>;

#[derive(Debug)]
enum Timestamp {
  At(DateTime<Utc>),
  Latest,
}

#[derive(Debug, Default, Deserialize)]
struct Identifier(String);

impl<'r> FromParam<'r> for Identifier {
  type Error = ValidationError<'r>;

  fn from_param(param: &'r str) -> Result<Self, Self::Error> {
    if param.chars().count() > 50 {
      return Err(ValidationError::validation("Invalid title length"));
    }

    Ok(Identifier(param.to_string()))
  }
}

impl<'r> FromParam<'r> for Timestamp {
  type Error = ParseError;

  fn from_param(param: &'r str) -> Result<Self, Self::Error> {
    let date = if param == "latest" {
      Timestamp::Latest
    } else {
      Timestamp::At(DateTime::from_str(param).unwrap_or_default())
    };

    Ok(date)
  }
}

async fn fetch_document(db: &mut DB, identifier: Identifier) -> Document {
  let title = identifier.0.replace("-", " ");

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
    identifier.0
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
  identifier: Identifier,
) -> Json<DocumentWithRevisions> {
  // Fetch latest document to fetch all the revisions
  let document = fetch_document(&mut db, identifier).await;
  // All revisions to the retrieved document
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

  // Combined Document and Revisions which makes responding a little easier
  Json(DocumentWithRevisions {
    document,
    revisions,
  })
}

#[get("/documents/<identifier>/<timestamp>")]
async fn get_document_at(
  mut db: DB,
  identifier: Identifier,
  timestamp: Timestamp,
) -> Json<Revision> {
  let document = fetch_document(&mut db, identifier).await;

  match timestamp {
    Timestamp::At(at) => {
      let date = at.to_rfc3339();

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
    // Cast a Document to a Revision as they're effectively the same thing
    // at this point in the query
    Timestamp::Latest => Json(Revision {
      id: String::new(),
      document_id: document.id,
      content: document.content,
      created_at: document.created_at,
    }),
  }
}

#[post("/documents/<identifier>", data = "<document>")]
async fn new_document(
  mut db: DB,
  identifier: Identifier,
  document: Json<NewDocument>,
) -> Json<Document> {
  let id = Uuid::new_v4().to_string();
  let title = identifier.0.replace("-", " ");

  query_as!(
    Document,
    r#"
      INSERT INTO documents
      VALUES(?, ?, ?, DATETIME(), DATETIME())
      RETURNING *
    "#,
    id,
    title,
    document.content,
  )
  .fetch_one(&mut *db)
  .await
  .map_or_else(|_| Json(Document::default()), |v| Json(v))
}

#[catch(404)]
fn not_found() -> &'static str {
  // This could be a JSON response but for the sake of the exercise I don't
  // think it matters too much
  "not found"
}

// Frustratingly, Rocket doesn't support query params throwing
// Instead, it simply defaults to 404
// I won't bother with the rabbit hole of trying to force the param validation
// 🤷
#[catch(500)]
fn default_catch(status: Status, req: &Request) -> String {
  // Errors could be handled more "gracefully", possibly in JSON, although
  // a basic text response will suffice
  format!("{status} ({}): {}", req.uri(), status.reason_lossy())
}

pub(super) fn routes() -> Vec<Route> {
  routes![list_documents, get_document, get_document_at, new_document]
}

pub(super) fn catchers() -> Vec<Catcher> {
  catchers![not_found, default_catch]
}
