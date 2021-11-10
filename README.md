# Overview

Realworld App using Rust, actix-web, and diesel.

## Getting Started

```zsh
$ docker-compose up -d

# TODO: add healtch check
# $ curl -X localhost:8080/api/healthcheck
# # ok
```

## e2e test

```zsh
# run e2e
$ APIURL=http://localhost:8080/api sh e2e/run-api-tests.sh
```

## LICENSE

MIT
