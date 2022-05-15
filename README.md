![logo](https://user-images.githubusercontent.com/26793088/168470794-337f3e7f-9c94-4cae-9505-1684b3251de5.png)

![CI](https://github.com/snamiki1212/realworld-v1-rust-actix-web-diesel/actions/workflows/ci.yml/badge.svg?branch=main)

# Overview

Realworld App using `Rust`, `actix-web`, and `diesel`.

## Getting Started

<details>
  <summary>Docker</summary>
  
```zsh
# ready
$ sh ./scripts/copy-env.sh
```

```zsh
$ docker compose up -d
```

```zsh
$ curl http://localhost:8080/api/healthcheck
# => OK
```

</details>

<details>
  <summary>Local</summary>
  
```zsh
# ready
$ sh ./scripts/copy-env.sh
```

```zsh
# start postgres
$ brew services start postgres

# start app
$ diesel setup
$ cargo watch
```

```zsh
$ curl http://localhost:8080/api/healthcheck
# => OK
```

  </details>

## E2E Test

Running E2E tests using [POSTMAN scripts](https://github.com/gothinkster/realworld/tree/main/api) on CI

```zsh
# run e2e
$ APIURL=http://localhost:8080/api zsh e2e/run-api-tests.sh
```

## Tech Stacks

- Rust Edition 2021
- ActixWeb 4.x
- Diesel 1.4.x

## Architecture

### Flow from Request to Response

```mermaid
sequenceDiagram
  actor Client
  participant Middleware as Middleware<br>/middleware/*
  participant Controller as Controller<br>/[feature]/api.rs
  participant Service as Service<br>/[feature]/service.rs
  participant DB

  Client ->> Middleware: request
  Middleware ->> Controller: -
  Controller ->> Controller: Assign to Request Object<br>(/[feature]/request.rs)
  Controller ->> Service: -
  Service ->> DB: -

  DB ->> Service: -
  Service ->> Controller: -
  Controller ->> Controller: Convert into Response Object<br>(/[feature]/response.rs)
  Controller ->> Client: response
```

## LICENSE

MIT
