use soph_server::{response::response, Response};

pub async fn index() -> Response {
    response().html("<h1>hello soph!</h1>")
}
