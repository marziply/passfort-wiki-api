#!/bin/bash

rm -rf assets/wiki.db

sqlite3 assets/wiki.db "" \
  -cmd ".mode csv" \
  -cmd ".import assets/data/documents.csv documents" \
  -cmd ".import assets/data/revisions.csv revisions"
