use rocket_contrib::json::Json;
use serde_json::Value;
use crate::datasource::db::Conn as DbConn;
use crate::post::repositories::implementations::models::{NewPost, Post};
use crate::profile::delivery::response::HttpResponse;
use rocket::State;
use crate::post::delivery::rest_adapter::Rest;
use crate::post::use_case::implementations::post::PostManagerImpl;
use crate::post::repositories::implementations::post::PostRepositoryImpl;
use rocket::http::Status;
use crate::profile::delivery::api_key::ApiKey;
use crate::post::delivery::post_request::PostRequest;

#[get("/posts", format = "application/json")]
pub fn post_get_all(
    key: ApiKey,
    adapter: State<Rest<PostManagerImpl<PostRepositoryImpl>>>,
) -> HttpResponse {
    match adapter.get_all_posts() {
        Ok(posts) => HttpResponse{ body: Json(json!(posts)), status: Status::Ok },
        Err(err) => HttpResponse{ body: Json(json!({"error": err.to_string()})), status: Status::InternalServerError },
    }

}

#[post("/posts", format = "application/json", data = "<new_post>")]
pub fn post_new(
    key: ApiKey,
    adapter: State<Rest<PostManagerImpl<PostRepositoryImpl>>>,
    new_post: Json<PostRequest>
) -> HttpResponse {
    match adapter.create_post(new_post) {
        Ok(post) => HttpResponse{ body: Json(json!(post)), status: Status::Created },
        Err(err) => HttpResponse{ body: Json(json!({"error": err.to_string()})), status: Status::InternalServerError },
    }
}

#[put("/posts/<post_id>", format = "application/json", data = "<new_post>")]
pub fn post_update(
    key: ApiKey,
    adapter: State<Rest<PostManagerImpl<PostRepositoryImpl>>>,
    post_id: i32,
    new_post: Json<PostRequest>
) -> HttpResponse {
    match adapter.update_post(new_post,post_id) {
        Ok(post) => HttpResponse{ body: Json(json!(post)), status: Status::Accepted },
        Err(err) => HttpResponse{ body: Json(json!({"error": err.to_string()})), status: Status::InternalServerError },
    }
}

#[delete("/posts/<post_id>", format = "application/json")]
pub fn delete_post(
    key: ApiKey,
    post_id: i32,
    adapter: State<Rest<PostManagerImpl<PostRepositoryImpl>>>,
) -> HttpResponse {
    match adapter.delete_post(post_id) {
        Ok(post) => HttpResponse{ body: Json(json!(post)), status: Status::Accepted },
        Err(err) => HttpResponse{ body: Json(json!({"error": err.to_string()})), status: Status::InternalServerError },
    }
}

#[get("/posts/<user_id>", format = "application/json")]
pub fn post_get(
    key: ApiKey,
    user_id: i32,
    adapter: State<Rest<PostManagerImpl<PostRepositoryImpl>>>
) -> HttpResponse {
    match adapter.get_posts_by_user_id(user_id){
        Ok(posts) => HttpResponse{ body: Json(json!(posts)), status: Status::Ok },
        Err(err) => HttpResponse{ body: Json(json!({"error": err.to_string()})), status: Status::InternalServerError },
    }
}
