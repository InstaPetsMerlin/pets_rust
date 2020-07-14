use rocket::http::RawStr;
use rocket_contrib::json::Json;
use serde_json::Value;

use crate::datasource::db::Conn as DbConn;
use crate::post::repositories::models::{NewPost, Post};
use crate::profile::delivery::api_key::ApiKey;

#[get("/posts", format = "application/json")]
pub fn post_get_all(conn: DbConn) -> Json<Value> {
    Json(json!({
        "posts": Post::get_all_posts(&conn),
    }))
}

#[post("/posts", format = "application/json", data = "<new_post>")]
pub fn post_new(conn: DbConn, new_post: Json<NewPost>) -> Json<Value> {
    let post = new_post.into_inner();
    let status = Post::insert_post(&post, &conn);
    Json(json!(
    {
        "status": status,
        "result": post,
    }))
}

#[get("/posts/<user_id>", format = "application/json")]
pub fn post_get(conn: DbConn, user_id: i32) -> Json<Value> {
    let posts = match Post::get_posts_by_user_id(user_id, &conn) {
        Ok(posts) => posts,
        Err(_) => {
            return Json(json!({
                "result":[],
            }));
        }
    };
    Json(json!({
        "result": posts,
    }))
}
