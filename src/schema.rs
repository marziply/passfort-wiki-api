use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Document {
  pub rowid: u64,
  pub title: String,
  pub content: String,
  pub created_at: DateTime<FixedOffset>,
  pub revisions: Vec<Revision>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Revision {
  pub rowid: u64,
  pub document_id: u64,
  pub content: String,
  pub created_at: DateTime<FixedOffset>,
}
