#[macro_use]
extern crate rocket;

use rocket_db_pools::{sqlx, Database};
use chrono::{Utc, DateTime};
use elucify::{model, Related};
use serde::{Serialize, Deserialize};

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

#[derive(Database)]
#[database("my_database")]
pub struct Db(sqlx::PgPool);

#[launch]
fn rocket() -> _ {
    rocket::build().attach(Db::init())
}
