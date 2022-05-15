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

Running API test using POSTMAN scripts here: [realworld/api at main · gothinkster/realworld · GitHub](https://github.com/gothinkster/realworld/tree/main/api).

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
