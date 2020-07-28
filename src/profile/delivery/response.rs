use rocket::{Response, response};
use rocket::http::{ContentType, Status};
use rocket::Request;
use rocket::response::Responder;
use rocket_contrib::json::JsonValue;

#[derive(Debug)]
pub struct HttpResponse {
    pub body: JsonValue,
    pub status: Status,
}

impl<'a> Responder<'a> for HttpResponse {
    fn respond_to(self, request: &Request) -> response::Result<'a> {
        Response::build_from(self.body.respond_to(&request).expect("could not generate response"))
            .header(ContentType::JSON)
            .ok()
    }
}

#[derive(Debug)]
pub struct ResponseError {
    pub body: JsonValue,
    pub status: Status,
}


#[derive(Debug)]
pub struct HttpResponseError {
    pub body: JsonValue,
    pub status: Status,
}

impl<'a> Responder<'a> for HttpResponseError {
    fn respond_to(self, request: &Request) -> response::Result<'a> {
        Response::build_from(self.body.respond_to(&request).expect("could not generate response"))
            .header(ContentType::JSON)
            .ok()
    }
}