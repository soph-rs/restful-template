use crate::{
    models::{post, prelude::Post},
    requests,
};
use axum::response::IntoResponse;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DbErr, EntityTrait, PaginatorTrait, QueryOrder};
use soph::{prelude::*, support::facades::Database};
use soph_server::{response, traits::RequestTrait, Request, Response};

pub async fn index(mut req: Request) -> Response {
    let pagination = req.query::<requests::post::Pagination>().await?;
    let limit = pagination.limit.unwrap_or(15);
    let page = pagination.page.unwrap_or(1);
    let db = Database::facade().connection();

    let paginator = Post::find()
        .order_by_desc(post::Column::Id)
        .into_json()
        .paginate(db, limit);

    let items = paginator.fetch_page(page - 1).await?;
    let items_and_pages = paginator.num_items_and_pages().await?;

    Ok(response::json()
        .paginate(page, limit, items, items_and_pages)
        .into_response())
}

pub async fn show(mut req: Request) -> Response {
    let id = req.path::<i32>().await?;
    let db = Database::facade().connection();

    let post = Post::find_by_id(id)
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotFound("post not found".to_string()))?;

    Ok(response::json().data(&post).into_response())
}

pub async fn store(req: Request) -> Response {
    let payload = req.json::<requests::post::Payload>().await?;
    let db = Database::facade().connection();

    let post = post::ActiveModel {
        title: Set(payload.title),
        text: Set(payload.text),
        ..Default::default()
    };

    post.save(db).await?;

    Ok(response::json().into_response())
}

pub async fn update(mut req: Request) -> Response {
    let id = req.path::<i32>().await?;
    let payload = req.json::<requests::post::Payload>().await?;
    let db = Database::facade().connection();

    let mut post: post::ActiveModel = Post::find_by_id(id)
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotFound("post not found".to_string()))?
        .into();

    post.title = Set(payload.title);
    post.text = Set(payload.text);
    post.update(db).await?;

    Ok(response::json().into_response())
}

pub async fn destroy(mut req: Request) -> Response {
    let id = req.path::<i32>().await?;
    let db = Database::facade().connection();

    let post: post::ActiveModel = Post::find_by_id(id)
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotFound("post not found".to_string()))?
        .into();

    post.delete(db).await?;

    Ok(response::json().into_response())
}
