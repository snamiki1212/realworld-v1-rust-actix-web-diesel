# Overview

Realworld App using Rust/actix-web.

## Getting Started

```zsh
# prerequisite
$ cargo install diesel_cli --no-default-features --features postgres

# run server
$ cargo run --bin conduit
```

## e2e test

```zsh
# run e2e
$ APIURL=http://localhost:8080/api sh e2e/run-api-tests.sh
```

## LICENSE

MIT
