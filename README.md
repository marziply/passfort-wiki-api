# PassFort Wikipedia API

A basic API for a service similar to Wikipedia.

## Setup

This application uses a basic SQLite embedded database, located at
`assets/wiki.db`. To initialise it, simply run:

```sh
./setup.sh
```

This will create a new database in the `assets` directory and seed it with
5 documents and 3 revisions per document.

Build the app with:

```sh
cargo build --release
```

Then you can run the app with:

```sh
./target/release/passfort-wiki-api
```

This application lacks logging in production so running `cargo run` instead
is probably better.

Testing is run with `cargo test`.

## Usage

All routes are defined in the [task](/TASK.md) document. For example:

```sh
curl -X GET -sSL http://localhost:8000/documents
```

This will respond with:

```json
[
  {
    "id": "11935251-7730-4b13-aa48-59a1bc20abc6",
    "title": "Class aptent taciti sociosqu ad litora",
    "content": "Torquent per conubia nostra, per inceptos himenaeos. Curabitur rhoncus dui tellus, at pulvinar purus semper eget.",
    "updated_at": "2022-08-15T21:37:29",
    "created_at": "2022-08-14T12:00:00"
  },
  {
    "id": "92f6cb5b-3045-4d9c-8b27-f92bbbba5baf",
    "title": "Aenean rutrum tristique viverra",
    "content": "In quis justo diam. In lectus nisi, scelerisque ut varius eget, dapibus eget lacus. Fusce venenatis urna et ultrices pharetra.",
    "updated_at": "2022-08-15T21:37:29",
    "created_at": "2022-08-14T12:00:00"
  },
  {
    "id": "281638af-3fcc-4e25-b462-29fc69a38a0a",
    "title": "Duis iaculis",
    "content": "Sodales tortor, a luctus erat placerat in. Duis ac malesuada sapien. Phasellus placerat dictum ligula sed feugiat.",
    "updated_at": "2022-08-15T21:37:29",
    "created_at": "2022-08-14T12:00:00"
  },
  {
    "id": "5cfa2e4c-7453-4f53-a35a-14ba9300145d",
    "title": "Ut non tempor risus, suscipit elementum nulla",
    "content": "Donec malesuada urna egestas tortor lobortis hendrerit. Nullam eget mattis magna. Nulla viverra molestie augue consectetur tristique. Sed sem ipsum, vulputate ac pretium vitae, porta eu sapien. Fusce aliquam accumsan augue at condimentum. Cras vitae scelerisque urna. Quisque at mauris dapibus, dapibus est ut, gravida nibh. Praesent ullamcorper sapien molestie orci mollis faucibus. Mauris vitae turpis libero. In sit amet turpis et diam imperdiet finibus. Duis diam magna, viverra nec porttitor vel, sollicitudin at ex. Fusce luctus imperdiet tincidunt.",
    "updated_at": "2022-08-15T21:37:29",
    "created_at": "2022-08-14T12:00:00"
  },
  {
    "id": "143bf8b7-103e-4d63-ac3b-fd21f9f2fb57",
    "title": "Sed tortor nulla, mollis vitae consequat vel, efficitur a ex",
    "content": "Vivamus tempor euismod magna sed convallis. Ut erat turpis, gravida non diam eget, euismod venenatis diam. Cras felis sapien, ornare vitae eleifend eget, aliquam vitae tortor. Nullam quis posuere nunc. Pellentesque mollis mattis augue ut malesuada. Vestibulum convallis ullamcorper sapien sit amet volutpat. Phasellus dignissim lectus felis. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos.",
    "updated_at": "2022-08-15T21:37:29",
    "created_at": "2022-08-14T12:00:00"
  }
]
```
