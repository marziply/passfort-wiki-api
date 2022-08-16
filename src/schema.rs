use chrono::NaiveDateTime;
use rocket::FromForm;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Wiki page document, always the most recent revision
// On edit, the current Document data is copied to `revisions`
// and then the Document is updated with the new content
#[derive(Debug, Default, Serialize, Deserialize, FromRow)]
pub struct Document {
  pub id: String,
  pub title: String,
  pub content: String,
  pub updated_at: NaiveDateTime,
  pub created_at: NaiveDateTime,
}

// Individual revision to a document
#[derive(Debug, Default, Serialize, Deserialize, FromRow)]
pub struct Revision {
  pub id: String,
  pub document_id: String,
  pub content: String,
  pub created_at: NaiveDateTime,
}

// Document and Revision joined as a single response object
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DocumentWithRevisions {
  pub document: Document,
  pub revisions: Vec<Revision>,
}

// Body data for creating a new Document
#[derive(Debug, Default, Deserialize, FromForm)]
pub struct NewDocument {
  #[field(validate = len(1..50))]
  pub content: String,
}
