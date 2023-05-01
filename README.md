# rust-poem-postgres-api-sample
Create an API with Rust, Poem and Diesel ORM based on Postgres DB features

Basic API include these end-points to perform operations over User table / entity:
* get users
* get user
* add user
* update user
* delete user

## Endpoints samples
### health
```
curl --request GET \
  --url http://localhost:6767/health
```
```
{"status":"server is running"}
``` 
### get users
```
curl --request GET \
  --url http://localhost:6767/users
```
```
[
	{
		"user_address": "c1234",
		"user_alias": "Cha cha cha St 12",
		"user_create_ts": null,
		"user_id": 1,
		"user_name": "clark",
		"user_update_ts": null
	},
	...
]
```
### get user
```
curl --request GET \
  --url http://localhost:6767/user/1
```
```
{
	"user_address": "c1234",
	"user_alias": "somestreet 9876",
	"user_create_ts": null,
	"user_id": 1,
	"user_name": "clark",
	"user_update_ts": null
}
```
### add user
```
curl --request POST \
  --url http://localhost:6767/user/add \
  --header 'Content-Type: application/json' \
  --data '{
	"user_name": "Bob Smith",
	"user_alias": "bobby123456",
	"user_address": "Lalala Avenue 876"
}'
```
```
{
	"user_address": "bobby123456",
	"user_alias": "Lalala Avenue 876",
	"user_create_ts": "2023-03-31T21:25:35",
	"user_id": 4,
	"user_name": "Bob Smith",
	"user_update_ts": "2023-03-31T21:25:35"
}
```
### update user
```
curl --request POST \
  --url http://localhost:6767/user/update/1 \
  --header 'Content-Type: application/json' \
  --data '{
	"user_address": "Cha cha cha St 12"
}'
```
```
{
	"user_address": "c1234",
	"user_alias": "Cha cha cha St 12",
	"user_create_ts": null,
	"user_id": 1,
	"user_name": "clark",
	"user_update_ts": null
}
```
### delete user
```
curl --request POST \
  --url http://localhost:6767/user/delete/4
```
```
{
	"status": "deleted ok"
}
```

## Docker 
### compose execution
* start your docker desktop agent
* important: you require a local-environment.env file just to set required environment variables:
  * POSTGRES_HOST
  * POSTGRES_PORT
  * POSTGRES_USER
  * POSTGRES_PASSWORD
  * POSTGRES_DB
```
 docker compose --env-file=./docker/local-environment.env -f ./docker/docker-compose-local.yaml up
```
### image build
* start your docker desktop agent
* IMPORTANT: you should set / export DATABASE_URL environment variable
```
docker build -t rust-poem-postgres-api-sample .
```
### image run
* start your docker desktop agent
```
docker run -p 6767:6767 --env-file ./docker/local-environment.env rust-poem-postgres-api-sample
```

## Run locally from command line
* required environment variables:
  * POSTGRES_HOST
  * POSTGRES_PORT
  * POSTGRES_USER
  * POSTGRES_PASSWORD
  * POSTGRES_DB
```
cargo run --color=always --package rust-poem-postgres-api-sample --bin rust-poem-postgres-api-sample
```

## How to...
### Get started
#### Rust install
* https://www.rust-lang.org/learn/get-started
* https://doc.rust-lang.org/book/ch01-01-installation.html

#### install MS C++ tools (instead of full Visual Studio IDE)
* https://visualstudio.microsoft.com/visual-cpp-build-tools/

#### install IntelliJ plugin (instead of Visual Studio Code plugin)
* https://plugins.jetbrains.com/plugin/8182-rust

### Create an Rust API using Poem
* https://docs.rs/poem/latest/poem/#quickstart

### Create modules (organize files)
* https://betterprogramming.pub/explaining-rusts-modules-420d38eed6c5

### Rust format and clippy
Remember configure your IDE (or run manually) these cargo commands just to improve your code
```
cargo fmt
```
```
cargo clippy
```
### Config Diesel ORM for PostgreSQL DB
#### install PostgreSQL libraries for Windows
* https://www.postgresql.org/download/windows/
#### set PG path
* setx PQ_LIB_DIR "[YOUR_POSTGRES_INSTALLATION_PATH]\lib"
* https://stackoverflow.com/questions/62708607/how-to-fix-diesel-cli-link-libpq-lib-error-with-postgres-tools-installed-in-dock
* add PQ_LIB_DIR to your PATH environment variable
* add PQ_BIN_DIR to your PATH environment variable
* https://taharmeijs.medium.com/diesel-rs-postgresql-linking-errors-on-windows-b769586664c1
#### reboot and execute clean cargo
* reboot your system (known issue, cargo does not read environment values properly)
* cargo clean
#### install Diesel ORM (for PostgreSQL DB)
* cargo install diesel_cli --no-default-features --features postgres
#### setup diesel
* diesel setup --database-url "postgres://[POSTGRES_USER]:[POSTGRES_PASSWORD]@[POSTGRES_HOST]:[POSTGRES_PORT]/[POSTGRES_DB]" (or use export DATABASE_URL environment variable)
#### setup and/or get entities from DB schema
* diesel migration generate create_user (if you need to create tables or others DB objects)
* diesel migration run (generates schema.rs with tables mapping)

#### use Diesel to access to DB
* https://betterprogramming.pub/rest-api-in-rust-step-by-step-guide-b8a6c5fcbff0
* https://levelup.gitconnected.com/diesel-a-rust-y-orm-e2c247e97835

#### DB Manager with Connection Pool
* https://stackoverflow.com/questions/75366208/the-trait-loadconnection-is-not-implemented-for-dieselpgconnection

#### important tips, tricks and comments:
* Diesel 2 - migration and changelog: https://github.com/diesel-rs/diesel/blob/5d78e38d21f8184dbbb73a0ecb714a204186c67d/guide_drafts/migration_guide.md

### Unit tests
* https://doc.rust-lang.org/book/ch11-01-writing-tests.html
#### execute test suites
* cargo test
* WIP

#### mocking dependencies
* https://docs.rs/mockall/latest/mockall/
* https://danielbunte.medium.com/a-guide-to-testing-and-mocking-in-rust-a73d022b4075
* WIP

#### test coverage
* https://doc.rust-lang.org/rustc/instrument-coverage.html
* https://blog.rng0.io/how-to-do-code-coverage-in-rust
* WIP

### Integration tests (tests folder)
* https://doc.rust-lang.org/book/ch11-03-test-organization.html
* WIP

#### execute test suites
* cargo test --test integration_test
* alternative: cargo test (full unit and integration tests execution)
