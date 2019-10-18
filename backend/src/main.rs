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
use rocket_contrib::serve::StaticFiles;

use rocket::fairing::AdHoc;
use rocket::{Rocket};

use diesel::SqliteConnection;

#[database("sqlite")]
pub struct Database(SqliteConnection);

embed_migrations!();

pub mod schema;

mod models;
use models::User;

use rocket::http::Status;
use rocket::response::status::Custom;
use serde::Serialize;
use std::net::SocketAddr;
use scrypt::scrypt_check;

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

#[post("/login", data = "<login_request>")]
fn login(
    remote_addr: SocketAddr,
    database: Database,
    login_request: Json<LoginRequest>,
) -> Result<Json<LoginResponse>, Custom<&'static str>> {
    if let Ok(user) = User::by_username(&database, &login_request.username) {
        if let Ok(_) = scrypt_check(&login_request.password, &user.password) {
            if let Ok(token) = user.generate_token(&database, &format!("{}", remote_addr.ip())) {
                return Ok(Json(LoginResponse { token }));
            }
        }
    }
    Err(Custom(
        Status::BadRequest,
        r#"{message: "Invalid credentials!"}"#,
    ))
}

fn run_db_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
    let database = Database::get_one(&rocket).expect("database connection");
    match embedded_migrations::run(&*database) {
        Ok(()) => Ok(rocket),
        Err(_) => Err(rocket),
    }
}

fn run_db_seeder(rocket: Rocket) -> Result<Rocket, Rocket> {
    if rocket
        .config()
        .get_bool("crate_admin_user")
        .unwrap_or(false)
    {
        let database = Database::get_one(&rocket).expect("database connection");
        let user = User::new("Admin", "admin");
        user.insert_into(&database).ok();
    }
    Ok(rocket)
}

fn main() {
    rocket::ignite()
        .attach(Database::fairing())
        .attach(AdHoc::on_attach("Database Migrations", run_db_migrations))
        .attach(AdHoc::on_attach("Database Seeder", run_db_seeder))
        .mount("/", routes![login])
        .mount("/", StaticFiles::from("../frontend/dist"))
        .launch();
}
