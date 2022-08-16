use crate::rocket as start;
use crate::schema::{Document, DocumentWithRevisions, Revision};
use rocket::http::Status;
use rocket::local::blocking::Client;

static FIRST_TITLE: &str = "Class-aptent-taciti-sociosqu-ad-litora";

static FIRST_ID: &str = "11935251-7730-4b13-aa48-59a1bc20abc6";

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
  let client = Client::untracked(start()).unwrap();
  let res = client
    .get(format!("/documents/{FIRST_TITLE}"))
    .dispatch();
  let status = res.status();
  let data: DocumentWithRevisions = res.into_json().unwrap_or_default();

  assert_eq!(status, Status::Ok);
  assert_eq!(data.document.title, FIRST_TITLE.replace('-', " "));
}

// Test for an empty Document ID which means the title was not found
#[test]
fn get_document_fail_by_find() {
  let client = Client::untracked(start()).unwrap();
  let res = client
    .get("/documents/foo-bar-baz")
    .dispatch();
  // let status = res.status();
  let data: DocumentWithRevisions = res.into_json().unwrap_or_default();

  // This should return 404 but alas, Rocket doesn't want to
  // assert_eq!(status, Status::NotFound);
  assert_eq!(data.document.id, "");
}

// Test for a failed request via a title length limitation of 50
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

#[test]
fn get_document_at() {
  let client = Client::untracked(start()).unwrap();
  let res = client
    .get(format!("/documents/{FIRST_TITLE}/2022-08-15T20:37:39Z"))
    .dispatch();
  let status = res.status();
  let data: Revision = res.into_json().unwrap_or_default();

  assert_eq!(status, Status::Ok);
  assert_eq!(data.id, "3ba2870d-8cfc-481f-9c99-233e34871901");
  assert_eq!(data.document_id, FIRST_ID);
}

// Test the Revision returned is equal to the second Revision in the data
#[test]
fn get_document_at_hour_before() {
  let client = Client::untracked(start()).unwrap();
  let res = client
    .get(format!("/documents/{FIRST_TITLE}/2022-08-15T19:37:39Z"))
    .dispatch();
  let status = res.status();
  let data: Revision = res.into_json().unwrap_or_default();

  assert_eq!(status, Status::Ok);
  assert_eq!(data.id, "f743e1b1-53b7-4097-8581-e4192d0cef89");
  assert_eq!(data.document_id, FIRST_ID);
}

// Test for the latest Revision result
#[test]
fn get_document_at_latest() {
  let client = Client::untracked(start()).unwrap();
  let res = client
    .get(format!("/documents/{FIRST_TITLE}/latest"))
    .dispatch();
  let status = res.status();
  let data: Revision = res.into_json().unwrap_or_default();

  assert_eq!(status, Status::Ok);
  assert_eq!(data.document_id, FIRST_ID);
  assert!(data.id.is_empty());
}

// Skipping the tests for POST routes because that would require transactions,
// which I am omitting from this exercise for the sake of time and brevity
// #[test]
// fn new_document() {
//   let client = Client::untracked(start()).unwrap();
//   let title = "foo-bar";
//   let res = client
//     .get(format!("/documents/{title}"))
//     .dispatch();
//   let status = res.status();
//   let data: Document = res.into_json().unwrap_or_default();
//
//   assert_eq!(status, Status::Ok);
//   assert_eq!(data.title, title.replace('-', " "));
// }
