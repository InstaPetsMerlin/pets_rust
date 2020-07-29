use rocket::{Response, response};
use rocket::http::{ContentType, Status};
use rocket::Request;
use rocket::response::Responder;
use rocket_contrib::json::{JsonValue, Json};
use serde_json::Value;

#[derive(Debug)]
pub struct HttpResponse {
    pub body: Json<Value>,
    pub status: Status,
}

impl<'a> Responder<'a> for HttpResponse {
    fn respond_to(self, request: &Request) -> response::Result<'a> {
        Response::build_from(self.body.respond_to(&request).expect("could not generate response"))
            .header(ContentType::JSON)
            .status(self.status)
            .ok()
    }
}