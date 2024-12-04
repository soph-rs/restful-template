use app::controllers;
use axum::routing::{delete, get, post, put};
use soph::prelude::*;
use soph_server::{middleware, router::Router, Server};

pub fn routes() -> Server {
    Server::new().register(
        axum::Router::new()
            .route("/", get(controllers::home::index))
            .merge(
                Router::new()
                    .route("/posts", get(controllers::post::index))
                    .route("/posts", post(controllers::post::store))
                    .route("/posts/:id", get(controllers::post::show))
                    .route("/posts/:id", put(controllers::post::update))
                    .route("/posts/:id", delete(controllers::post::destroy))
                    .route("/auth/login", post(controllers::auth::login)),
            )
            .merge(
                Router::default()
                    .middleware::<middleware::Auth>()
                    .route("/auth/user", get(controllers::auth::user)),
            ),
    )
}
