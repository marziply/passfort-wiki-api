use crate::rocket as start;
use crate::schema::{Document, DocumentWithRevisions};
use rocket::http::Status;
use rocket::local::blocking::Client;

#[test]
fn list_documents() {
  let client = Client::untracked(start()).unwrap();
  let res = client.get("/documents").dispatch();
  let status = res.status();
  let data: Vec<Document> = res.into_json().unwrap_or_default();

  assert_eq!(status, Status::Ok);
  assert_eq!(data.len(), 5);
}

#[test]
fn get_document() {
  let title = "Class-aptent-taciti-sociosqu-ad-litora";
  let client = Client::untracked(start()).unwrap();
  let res = client
    .get(format!("/documents/{title}"))
    .dispatch();
  let status = res.status();
  let data: DocumentWithRevisions = res.into_json().unwrap_or_default();

  assert_eq!(status, Status::Ok);
  assert_eq!(data.document.title, title.replace('-', " "));
}

#[test]
fn get_document_fail_by_find() {
  let client = Client::untracked(start()).unwrap();
  let res = client
    .get("/documents/foo-bar-baz")
    .dispatch();
  let status = res.status();
  let data: DocumentWithRevisions = res.into_json().unwrap_or_default();

  assert_eq!(status, Status::NotFound);
  assert_eq!(data.document.id, "");
}

#[test]
fn get_document_fail_by_title_len() {
  let title = "a".repeat(51);
  let client = Client::untracked(start()).unwrap();
  let res = client
    .get(format!("/documents/{title}"))
    .dispatch();
  let status = res.status();
  let data: DocumentWithRevisions = res.into_json().unwrap_or_default();

  assert_eq!(status, Status::NotFound);
  assert_eq!(data.document.id, "");
}
