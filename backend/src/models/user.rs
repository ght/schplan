use scrypt::{ScryptParams, scrypt_simple};

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use diesel::prelude::*;

use crate::schema::users::{self, dsl};
use crate::Database;

#[table_name = "users"]
#[derive(Identifiable, Serialize, Queryable, Insertable, Debug, Clone)]
pub struct User {
    pub id: Option<i32>,
    pub token: Option<String>,
    pub ip: Option<String>,
    pub username: String,
    pub password: String,
}

impl User {
    pub fn new<U: Into<String>, P: Into<String>>(username: U, password: P) -> User {
        let params = ScryptParams::new(0, 8, 1).unwrap();
        let username = username.into();
        let password = scrypt_simple(&password.into(), &params).expect("OS RNG should not fail");

        Self {
            id: None,
            token: None,
            ip: None,
            username,
            password,
        }
    }

    pub fn generate_token(&self, database: &Database, ip: &str) -> diesel::QueryResult<String> {
        let token: String = thread_rng().sample_iter(&Alphanumeric).take(64).collect();

        diesel::update(self)
            .set((users::token.eq(&token), users::ip.eq(&ip)))
            .execute(&database.0)?;

        Ok(token)
    }

    pub fn insert_into(self, database: &Database) -> diesel::QueryResult<User> {
        diesel::insert_into(dsl::users)
            .values(&self)
            .execute(&database.0)?;
        dsl::users.order(users::id.desc()).first(&database.0)
    }

    pub fn by_username(database: &Database, username: &str) -> diesel::QueryResult<User> {
        dsl::users
            .filter(users::username.eq(&username))
            .first(&database.0)
    }
}