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


use rocket_contrib::json::Json;
use serde_json::Value;

use post::delivery::routes as postRoutes;
use profile::delivery::rest_adapater::*;
use profile::delivery::routes as profileRoutes;


use crate::profile::repositories::implementations::profile::ProfileRepositoryImpl;
use crate::profile::use_case::implementations::profile_manager_impl::ProfileManagerImpl;
use crate::post::use_case::implementations::post::PostManagerImpl;
use crate::post::repositories::implementations::post::PostRepositoryImpl;
use crate::post::delivery::rest_adapter::Rest;


mod datasource;
mod post;
mod profile;

fn rocket() -> rocket::Rocket {
    let profile_repo = ProfileRepositoryImpl::new();
    let profile_manager = ProfileManagerImpl::new(profile_repo);
    let profile_rest_adapter = ProfileRestAdapter::new(profile_manager);
    let post_repo = PostRepositoryImpl::new();
    let post_manager = PostManagerImpl::new(post_repo);
    let post_rest_adapter = Rest::new(post_manager);
    rocket::ignite()
        .manage(profile_rest_adapter)
        .manage(post_rest_adapter)
        .mount("/",routes![health])
        .mount(
        "/api/v1/",
        routes![
            profileRoutes::get_all,  //Get all users,
            profileRoutes::new_user, // Create a new User,
            profileRoutes::get_user, // get a user by username,
            profileRoutes::login,
            profileRoutes::update_user,
            profileRoutes::delete_user,
            postRoutes::post_get_all,
            postRoutes::post_new,
            postRoutes::post_get
        ],
    )
}

fn main() {
    rocket().launch();
}

#[get("/", format = "application/json")]
pub fn health() -> Json<Value> {
    Json(json!({
        "result": String::from("hello, world"),
    }))
}
