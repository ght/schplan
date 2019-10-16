#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate serde;

use rocket_contrib::json::Json;
use diesel::prelude::*;
use diesel::SqliteConnection;

use rocket::Rocket;
use rocket::fairing::AdHoc;
use rocket::request::{Form, FlashMessage};
use rocket::response::{Flash, Redirect};

embed_migrations!();

#[database("sqlite")]
struct Database(SqliteConnection);

mod schema;

use schema::users::{self, dsl};

#[table_name = "users"]
#[derive(Serialize, Queryable, Insertable, Debug, Clone)]
pub struct User {
    pub id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
}

#[get("/users/<id>")]
fn user(database: Database, id: i32) -> Option<Json<User>> {
    if let Ok(user) = dsl::users.find(id).get_result::<User>(&database.0) {
        Some(Json(user))
    } else {
        None
    }
}

fn run_db_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
	let database = Database::get_one(&rocket).expect("database connection");
	match embedded_migrations::run(&*database) {
		Ok(()) => Ok(rocket),
		Err(e) => Err(rocket)
	}
}

fn main() {
    rocket::ignite()
        .attach(Database::fairing())
		.attach(AdHoc::on_attach("Database Migrations", run_db_migrations))
        .mount("/", routes![user])
        .launch();
}
