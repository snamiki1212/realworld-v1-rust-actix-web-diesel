{ config, lib, pkgs, ... }:

let
  postgres-user = "postgres";
  postgres-password = "postgres";
  postgres-port = 5432;
  postgres-db = "realworld-rust-actix-web";
in
{

  packages = with pkgs; [
    rustfmt
    (diesel-cli.override {
      sqliteSupport = false;
      mysqlSupport = false;
      postgresqlSupport = true;
    })
    cargo-watch
  ];

  languages.rust.enable = true;

  scripts.run-app.exec = ''
    cd $DEVENV_ROOT
    cargo install --path .
    diesel setup
    cargo watch --exec run
  '';

  services.postgres = {
    enable = true;
    port = postgres-port;
    listen_addresses = "127.0.0.1";
    initialDatabases = [
      {
        name = postgres-db;
        schema = pkgs.writeText "init.sql" ''
          create user "${postgres-user}" with password '${postgres-password}' superuser;
        '';
      }
    ];
  };

  env = {
    DATABASE_URL = "postgres://${postgres-user}:${postgres-password}@localhost:${toString postgres-port}/${postgres-db}";
    FRONTEND_ORIGIN = "http://localhost:3000";
    APIURL = "http://localhost:8080/api";
  };

  process.implementation = "process-compose";
  processes.app = {
    exec = "run-app";
    process-compose.depends_on.postgres.condition = "process_ready";
  };

  pre-commit.hooks = {
    clippy.enable = true;
    rustfmt.enable = true;
    nixpkgs-fmt.enable = true;
    shellcheck.enable = true;
  };
}
