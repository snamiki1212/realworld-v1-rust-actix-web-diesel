# Overview

Realworld App using `Rust`, `actix-web`, and `diesel`.

## Getting Started

```zsh
$ docker-compose up -d
$ curl http://localhost:8080/api/healthcheck
# => OK
```

## e2e test

```zsh
# run e2e
$ APIURL=http://localhost:8080/api sh e2e/run-api-tests.sh
```

## Architecture

### Req to res flow

![Req to res flow](http://www.plantuml.com/plantuml/proxy?cache=no&src=https://raw.githubusercontent.com/snamiki1212/realworld-rust-actix-web/main/doc/data_flow.pu)

## LICENSE

MIT
