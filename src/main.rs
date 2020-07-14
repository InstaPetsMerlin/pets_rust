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

use profile::delivery::routes::*;

use crate::datasource::db;

mod datasource;
mod profile;

fn rocket() -> rocket::Rocket {
    rocket::ignite().manage(db::init_pool()).mount(
        "/api/v1/",
        routes![get_all, new_user, get_user, login, health],
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
