use addon_common::{JsonListResponse, JsonResponse, ListResponse, WrappingResponse};
use axum::{
    extract,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::{
    models::{BlogModel, NewPostModel, PostModel, PostStatus},
    PostId, Result,
};

pub fn routes() -> Router<SqlitePool> {
    Router::new()
        .route("/:instance/overview", get(get_overview))
        .route("/:instance/analytics", get(get_analytics))
        // .route("/:instance/categories", get(get_category_list))
        // .route("/:instance/comments", get(get_comment_list))
        // .route("/:instance/tags", get(get_tag_list))
        .route("/:instance/posts", get(get_post_list))
        .route("/:instance/post", post(create_post))
        .route("/:instance/post/:post_id", get(get_post).post(update_post))
}

async fn get_overview(
    extract::Path(instance_id): extract::Path<Uuid>,
    extract::State(db): extract::State<SqlitePool>,
) -> Result<JsonResponse<serde_json::Value>> {
    let mut acq = db.acquire().await?;

    let Some(_blog) = BlogModel::find_one_by_instance_id(instance_id, &mut *acq).await? else {
        return Err(eyre::eyre!("Addon not found"))?;
    };

    Ok(Json(WrappingResponse::okay(serde_json::json!({
        //
    }))))
}

async fn get_post_list(
    extract::Path(instance_id): extract::Path<Uuid>,
    extract::State(db): extract::State<SqlitePool>,
) -> Result<JsonListResponse<PostModel>> {
    let mut acq = db.acquire().await?;

    let Some(blog) = BlogModel::find_one_by_instance_id(instance_id, &mut *acq).await? else {
        return Err(eyre::eyre!("Addon not found"))?;
    };

    let posts = PostModel::find_by_blog_id(blog.id, &mut *acq).await?;

    Ok(Json(WrappingResponse::okay(ListResponse::all(posts))))
}

async fn get_analytics(
    extract::Path(instance_id): extract::Path<Uuid>,
    extract::State(db): extract::State<SqlitePool>,
) -> Result<JsonResponse<serde_json::Value>> {
    let mut acq = db.acquire().await?;

    let Some(_blog) = BlogModel::find_one_by_instance_id(instance_id, &mut *acq).await? else {
        return Err(eyre::eyre!("Addon not found"))?;
    };

    Ok(Json(WrappingResponse::okay(serde_json::json!({
        //
    }))))
}

#[derive(Deserialize)]
struct CreatePostJson {
    title: String,
    content: serde_json::Value,
}

async fn create_post(
    extract::Path(instance_id): extract::Path<Uuid>,
    extract::State(db): extract::State<SqlitePool>,
    extract::Json(CreatePostJson { title, content }): extract::Json<CreatePostJson>,
) -> Result<JsonResponse<serde_json::Value>> {
    let mut acq = db.acquire().await?;

    let Some(blog) = BlogModel::find_one_by_instance_id(instance_id, &mut *acq).await? else {
        return Err(eyre::eyre!("Addon not found"))?;
    };

    let post = NewPostModel {
        blog_id: blog.id,
        slug: None,
        title,
        content,
        status: PostStatus::Draft,
    }
    .insert(&mut *acq)
    .await?;

    Ok(Json(WrappingResponse::okay(serde_json::json!({
        "id": post.id,
        "slug": post.slug,
    }))))
}

async fn get_post(
    extract::Path((instance_id, post_id)): extract::Path<(Uuid, i64)>,
    extract::State(db): extract::State<SqlitePool>,
) -> Result<JsonResponse<serde_json::Value>> {
    let mut acq = db.acquire().await?;

    let Some(blog) = BlogModel::find_one_by_instance_id(instance_id, &mut *acq).await? else {
        return Err(eyre::eyre!("Addon not found"))?;
    };

    let Some(post) = PostModel::find_one_by_id(PostId::from(post_id), &mut *acq).await? else {
        return Err(eyre::eyre!("Post not found"))?;
    };

    Ok(Json(WrappingResponse::okay(serde_json::json!({
        "id": post.id,
        "slug": post.slug,
        "title": post.title,
        "content": post.content,
        "status": post.status,
    }))))
}

#[derive(Deserialize)]
struct UpdatePostJson {
    title: Option<String>,
    content: Option<serde_json::Value>,
    status: Option<PostStatus>,
    slug: Option<String>,
}

async fn update_post(
    extract::Path((instance_id, post_id)): extract::Path<(Uuid, i64)>,
    extract::State(db): extract::State<SqlitePool>,
    extract::Json(UpdatePostJson {
        title,
        content,
        status,
        slug,
    }): extract::Json<UpdatePostJson>,
) -> Result<JsonResponse<serde_json::Value>> {
    let mut acq = db.acquire().await?;

    let Some(_blog) = BlogModel::find_one_by_instance_id(instance_id, &mut *acq).await? else {
        return Err(eyre::eyre!("Addon not found"))?;
    };

    let Some(mut post) = PostModel::find_one_by_id(PostId::from(post_id), &mut *acq).await? else {
        return Err(eyre::eyre!("Post not found"))?;
    };

    if let Some(title) = title {
        post.title = title;
    }

    if let Some(content) = content {
        post.content.0 = content;
    }

    if let Some(status) = status {
        post.status = status as u8 as i32;
    }

    if let Some(slug) = slug {
        post.slug = Some(slug);
    }

    post.update(&mut *acq).await?;

    Ok(Json(WrappingResponse::okay(serde_json::json!({
        "id": post.id,
        "slug": post.slug,
    }))))
}
