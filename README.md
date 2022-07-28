# Elucify

Elucify is a simple Postgres ORM for Rust apps using [Rocket](https://github.com/SergioBenitez/Rocket).

## Description

Currently, the project supports integrating Rust-based models with existing Postgres databases. It supporings handling models when creating, reading, filtering, updating, deleting, converting to JSON, and easily querying related models.

## Inspiration

Elucify was created mostly because I found that most well established, robust Rust ORMs did not provide much high level ease of use that I've seen in ORMs from other languages. Mainly, I wanted something that would abstract away the SQL logic and allow users to plainly use Rust structs with macro-given function implementations.

## Usage

First, a `Db` struct must be defined at the crate level, like this example from the [Rocket documentation](https://api.rocket.rs/v0.5-rc/rocket_db_pools/index.html):

```rust
use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("sqlite_logs")]
pub struct Logs(sqlx::SqlitePool);

#[launch]
fn rocket() -> _ {
		rocket::build().attach(Logs::init())
}
```

### Creating Models

Next, models can be defined easily from anywhere as so:

```rust
#[model]
pub struct User {
	pub username: String,
	pub email: String,
	pub created_at: DateTime<Utc>,
	pub last_login: DateTime<Utc>,
}

#[model(table = "credentials")]
#[derive(Related)]
pub struct Credentials {
	#[foreign(type = "User")]
	pub user_id: i32,
	pub password: String,
}
```

Notice that the `id` column is omitted, as it is added implicitly. Structs that have foreign keys (such as `user_id`) must derive `Related`. By default, table names are the equivalent to the lowercase struct name appended with an 's', so that `User` references the table `users`.

### Model Features

When a model is created in Elucify, the following functions are given:

```rust
// Return table name (such as "users")
table() -> &'static str

// Find the model in the database with given id
find(id: i32, mut db: Connection<Db>) -> (Option<Self>, Connection<Db>)

// Find the first model with some field constraint
find_where(field: &str, value: &String, mut db: Connection<Db>) -> (Option<Self>, Connection<Db>)

// Identical to find(i32, Connection<Db>)
read(id: i32, mut db: Connection<Db>) -> (Option<Self>, Connection<Db>)

// Delete a record from the database
delete(id: i32, mut db: Connection<Db>) -> (Result<u64, Error>, Connection<Db>)

// Construct a new model record from given data
new(...fields) -> Self

// Insert or update a record to the database
save(&self, mut db: Connection<Db>) -> (Option<Self>, Connection<Db>)

// Convert a model record to JSON
json(self) -> rocket::serde::json::Json<Self>
```

For models that are related to other models, additional functions are implemented. With the example of `User` and `Credentials`, the following is given:

```rust
// Get all credentials records that reference this user
user.find_credentials(&self, mut db: Connection<Db>) -> (Vec<Credentials>, Connection<Db>)

// Get the user record associated with these credentials
credentials.get_user(&self, mut db: Connection<Db>) -> (Option<User>, Connection<Db>)
```

## Future Work

The project is in a basic form, created from a subset of another project I am working on. In the future, I plan to allow Elucify to use any database format and not rely on `rocket::serde` for JSON conversion.
