use addon_common::{JsonResponse, WrappingResponse};
use axum::{extract, routing::get, Json, Router};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::{models::BlogModel, Result};

pub fn routes() -> Router<SqlitePool> {
    Router::new().route("/:instance/overview", get(blog_overview))
}

async fn blog_overview(
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
