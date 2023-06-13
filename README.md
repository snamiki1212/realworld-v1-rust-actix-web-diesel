<a href="https://github.com/snamiki1212/realworld-v1-rust-actix-web-diesel"><img src="https://user-images.githubusercontent.com/26793088/168470794-337f3e7f-9c94-4cae-9505-1684b3251de5.png" alt="header"></a>

<a href="https://github.com/snamiki1212/realworld-v1-rust-actix-web-diesel/actions?query=branch%3Amain"><img src="https://github.com/snamiki1212/realworld-v1-rust-actix-web-diesel/actions/workflows/ci.yml/badge.svg?branch=main" alt="badge" /></a>

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
$ brew services start postgresql

# start app
$ diesel setup
$ cargo run
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
- Diesel 2.x

## Architecture

- Clean Architecture
- DI container using Constructor Injection with dynamic dispatch (`/src/di.rs`)

```mermaid
sequenceDiagram
  actor Client
  autonumber
  participant Route as Middleware + Route <br><br>/src/app/drivers/{middlewares, routes}
  participant Controller as Controller<br><br>/src/app/features/[feature]/controllers.rs
  participant Presenter as Presenter<br><br>/src/app/features/[feature]/presenters.rs
  participant Usecase as Usecase<br><br>/src/app/features/[feature]/usecases.rs
  participant Repository as Repository<br><br>/src/app/features/[feature]/repositories.rs
  participant Entity as Entity<br><br>/src/app/features/[feature]/entities.rs
  participant DB

  %% left to right
  Client -->> Route: Request
  Route ->> Controller: <br>
  Controller ->> Usecase: <br>
  Usecase ->> Repository: <br>
  Repository ->> Entity: <br>
  Entity ->> DB: <br>

  %% right to left
  DB ->> Entity: <br>
  Entity ->> Repository: <br>
  Repository ->> Usecase: <br>
  Usecase ->> Presenter: <br>
  Presenter -->> Client: Response
```

## LICENSE

MIT
