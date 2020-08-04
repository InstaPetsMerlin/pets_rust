use rocket_contrib::json::Json;
use serde_json::Value;
use crate::datasource::db::Conn as DbConn;
use crate::post::repositories::models::{NewPost, Post};
use crate::profile::delivery::response::HttpResponse;
use rocket::State;
use crate::post::delivery::rest_adapter::Rest;
use crate::post::use_case::post::PostManager;
use crate::post::repositories::post::PostRepository;
use crate::post::use_case::implementations::post::PostManagerImpl;
use crate::post::repositories::implementations::post::PostRepositoryImpl;
use crate::profile::delivery::api_key::ApiKey;
use crate::post::errors::PostError;
use rocket::http::Status;

#[get("/posts", format = "application/json")]
pub fn post_get_all(
    adapter: State<Rest<PostManagerImpl<PostRepositoryImpl>>>,
) -> HttpResponse {
    match adapter.get_all_posts() {
        Ok(posts) => HttpResponse{ body: Json(json!(posts)), status: Status::Ok },
        Err(err) => HttpResponse{ body: Json(json!({"error": err.to_string()})), status: Status::Ok },
    }

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
