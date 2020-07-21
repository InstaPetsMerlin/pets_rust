#![feature(plugin, const_fn, decl_macro, proc_macro_hygiene)]
#![allow(proc_macro_derive_resolution_fallback, unused_attributes)]
#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use std::process::Command;

use rocket_contrib::json::Json;
use serde_json::Value;

use post::delivery::routes::*;
use profile::delivery::rest_adapater::*;
use profile::delivery::routes::*;

use crate::datasource::db;
use crate::datasource::db::Conn;
use crate::profile::repositories::implementations::profile::ProfileRepositoryImpl;
use crate::profile::use_case::implementations::profile_manager_impl::ProfileManagerImpl;
use diesel::PgConnection;
use r2d2::{Error, PooledConnection};
use r2d2_diesel::ConnectionManager;

mod datasource;
mod post;
mod profile;

fn rocket() -> rocket::Rocket {
    let profile_repo = ProfileRepositoryImpl::new();
    let manager = ProfileManagerImpl::new(profile_repo);
    let profile_rest_adapter = ProfileRestAdapter::new(manager);

    rocket::ignite()
        .manage(db::init_pool())
        .manage(profile_rest_adapter)
        .mount(
        "/api/v1/",
        routes![
            get_all,  //Get all users,
            new_user, // Create a new User,
            get_user, // get a user by username,
            login,
            health,
            post_get_all,
            post_new,
            post_get
        ],
    )
}

fn main() {
    let _output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cd ui && npm start"])
            .spawn()
            .expect("Failed to start UI Application")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("cd ui && npm start")
            .spawn()
            .expect("Failed to start UI Application")
    };
    rocket().launch();
}

#[get("/", format = "application/json")]
pub fn health() -> Json<Value> {
    Json(json!({
        "result": String::from("hello, world"),
    }))
}
