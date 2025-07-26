use addon_common::{InstallResponse, JsonResponse, RegisterNewJson, WrappingResponse};
use axum::{extract, routing::post, Json, Router};
use sqlx::SqlitePool;

use crate::{models::NewBlogModel, Result};

pub fn routes() -> Router<SqlitePool> {
    Router::new().route("/", post(post_install))
}

async fn post_install(
    extract::State(db): extract::State<SqlitePool>,
    extract::Json(value): extract::Json<RegisterNewJson>,
) -> Result<JsonResponse<InstallResponse>> {
    let mut acq = db.acquire().await?;

    let _model = NewBlogModel {
        instance_id: value.instance_id,
        external_website_id: value.website_id,
        external_member_id: value.owner_id,
        name: value.website.name,
    }
    .insert(&mut acq)
    .await?;

    Ok(Json(WrappingResponse::okay(InstallResponse::Complete)))
}
