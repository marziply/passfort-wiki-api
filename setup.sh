#!/bin/bash

target="assets/wiki.db"
query="
  CREATE TABLE documents(
    id TEXT NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    updated_at DATETIME NOT NULL,
    created_at DATETIME NOT NULL,
    PRIMARY KEY (id)
  );
  create table revisions(
    id TEXT NOT NULL,
    document_id TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at DATETIME NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (document_id)
      REFERENCES documents (id)
      ON DELETE CASCADE
  );
"

rm -rf $target

sqlite3 \
  "$target" \
  "$query" \
  ".mode csv" \
  ".import assets/data/documents.csv documents" \
  ".import assets/data/revisions.csv revisions" \
  ".exit"
