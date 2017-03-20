#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;
extern crate chrono;
extern crate r2d2;
extern crate r2d2_diesel;

extern crate serde_json;

mod orm;
use orm::*;

// Server Imports
// Used to Setup DB Pool
use rocket::request::{Outcome, FromRequest};
use rocket::Outcome::{Success, Failure};
use rocket::http::Status;

// Used for Routes
use rocket::Request;
use rocket::response::NamedFile;
use rocket_contrib::JSON;

// Std Imports
use std::path::{Path, PathBuf};

// DB Imports
use diesel::prelude::*;
use diesel::update;
use diesel::sqlite::SqliteConnection;
use r2d2::{Pool, PooledConnection, GetTimeout};
use r2d2_diesel::ConnectionManager;

// DB Items {{{
lazy_static! {
    pub static ref DB_POOL: Pool<ConnectionManager<SqliteConnection>> = create_db_pool();
}

pub struct DB(PooledConnection<ConnectionManager<SqliteConnection>>);

impl DB {
    pub fn conn(&self) -> &SqliteConnection {
        &*self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for DB {
    type Error = GetTimeout;
    fn from_request(_: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        match DB_POOL.get() {
            Ok(conn) => Success(DB(conn)),
            Err(e) => Failure((Status::InternalServerError, e)),
        }
    }
}
// }}}

// Routes {{{
#[get("/")]
fn get_products(db: DB) -> JSON<Product> {
    JSON(Product::new())
}

#[put("/")]
fn put_products(db: DB) -> JSON<Product> {
    JSON(Product::new())
}

// }}}

#[derive(Deserialize, Serialize)]
pub struct Product {
    pub name:           String,
    pub artifact:       String,
    pub environment:    String,
    pub version:        String,
    pub deployed:       chrono::NaiveDateTime,
}

impl Product {
    fn new() -> Self {
        Product {
            name:           String::new(),
            artifact:       String::new(),
            environment:    String::new(),
            version:        String::new(),
            deployed:       chrono::NaiveDateTime::now(),
        }
    }
}
fn main() {

    rocket::ignite().mount("/", routes![get_products, put_products]).launch();
}
