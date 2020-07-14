use rocket::http::RawStr;
use rocket_contrib::json::Json;
use serde_json::Value;

use crate::datasource::db::Conn as DbConn;
use crate::post::repositories::models::{NewPost, Post};
use crate::profile::delivery::api_key::ApiKey;

#[get("/posts", format = "application/json")]
pub fn get_all(_conn: DbConn) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": "post",
    }))
}

#[post("/posts", format = "application/json", data = "<new_post>")]
pub fn new_post(conn: DbConn, new_post: Json<NewPost>) -> Json<Value> {
    let post = new_post.into_inner();
    let status = Post::insert_post(&post, &conn);
    Json(json!(
    {
        "status": status,
        "result": post,
    }))
}

#[get("/posts/<post_id>", format = "application/json")]
pub fn get_post(_conn: DbConn, post_id: &RawStr, key: ApiKey) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": "post",
    }))
}
