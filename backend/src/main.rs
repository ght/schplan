#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;

use rocket_contrib::json::Json;
use diesel::prelude::*;
use diesel::SqliteConnection;

use rocket::Rocket;
use rocket::fairing::AdHoc;
use rocket::request::{Form, FlashMessage};
use rocket::response::{Flash, Redirect};

#[database("sqlite")]
struct Database(SqliteConnection);

mod schema;

use schema::{users, users::dsl::users as all_users};

#[table_name = "users"]
#[derive(Serialize, Queryable, Insertable, Debug, Clone)]
pub struct User {
    pub id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
}

#[get("/users/<id>")]
fn user(database: Database, id: i32) -> Option<Json<User>> {
    if let Ok(user) = all_users.find(id).get_result::<User>(&database.0) {
        Some(Json(user))
    } else {
        None
    }
}

fn main() {
    rocket::ignite()
        .attach(Database::fairing())
        .mount("/", routes![user])
        .launch();
}
