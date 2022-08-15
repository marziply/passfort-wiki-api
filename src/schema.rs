use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Document {
  pub title: String,
  pub text: String,
  pub created_at: DateTime<FixedOffset>,
  pub revisions: Vec<Document>,
}
