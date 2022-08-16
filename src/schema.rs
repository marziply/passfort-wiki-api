use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Default, Serialize, FromRow)]
pub struct Document {
  pub id: String,
  pub title: String,
  pub content: String,
  pub updated_at: NaiveDateTime,
  pub created_at: NaiveDateTime,
}

#[derive(Debug, Default, Serialize, FromRow)]
pub struct Revision {
  pub id: String,
  pub document_id: String,
  pub content: String,
  pub created_at: NaiveDateTime,
}

#[derive(Debug, Default, Serialize)]
pub struct DocumentWithRevisions {
  pub document: Document,
  pub revisions: Vec<Revision>,
}
