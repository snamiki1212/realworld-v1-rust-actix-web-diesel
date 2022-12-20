<a href="https://github.com/snamiki1212/realworld-v1-rust-actix-web-diesel"><img src="https://user-images.githubusercontent.com/26793088/168470794-337f3e7f-9c94-4cae-9505-1684b3251de5.png" alt="header"></a>

<a href="https://github.com/snamiki1212/realworld-v1-rust-actix-web-diesel/actions?query=branch%3Amain"><img src="https://github.com/snamiki1212/realworld-v1-rust-actix-web-diesel/actions/workflows/ci.yml/badge.svg?branch=main" alt="badge" /></a>

# Overview

Realworld App using `Rust`, `actix-web`, and `diesel`.

## Getting Started

  
First-time setup:
```zsh
# Install Nix Flake + direnv as per https://devenv.sh/getting-started
$ direnv allow
```

Start postgres and compile and run project:
```zsh
$ devenv up
```

Try out endpoint:
```zsh
$ curl $APIURL/healthcheck
# => OK
```

## E2E Test

Running E2E tests using [POSTMAN scripts](https://github.com/gothinkster/realworld/tree/main/api) on CI

```zsh
# run e2e
$ $DEVENV_ROOT/e2e/run-api-tests.sh
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
