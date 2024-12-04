use serde_json::json;
use soph::{
    prelude::*,
    support::facades::{Auth, AuthError, Hash},
};
use soph_server::{response::response, traits::RequestTrait, Request, Response};

pub async fn login(_req: Request) -> Response {
    // ...user verify
    let password = "password";

    if !Hash::facade().check(
        password,
        "$argon2id$v=19$m=19456,t=2,p=1$7r6mr7wQ1VLhOo/pPI2bEw$dNknO9LGRm5cHGi86cVYj/cXTLzJ1AdZ/\
         vJ6jcIRQiw",
    ) {
        return Err(AuthError::Unauthorized("Unauthorized".to_string()))?;
    }

    let token = Auth::facade().authorize("1".to_string())?;

    response().json(json!({"token": token}))
}

pub async fn user(req: Request) -> Response {
    let user = req
        .user()
        .ok_or_else(|| AuthError::Unauthorized("Unauthorized".to_string()))?;

    // let user = Auth::facade().user(&req)?;

    response().json(json!({"user": user}))
}
