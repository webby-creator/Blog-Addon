//! Conversions to CMS format

use addon_common::{JsonListResponse, ListResponse, WrappingResponse};
use axum::{extract, routing::get, Json, Router};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::Result;

pub fn routes() -> Router<SqlitePool> {
    Router::new().route("/:instance/query", get(get_query))
}

async fn get_query(
    extract::Path(_inst_id): extract::Path<Uuid>,
    // extract::State(db): extract::State<SqlitePool>,
) -> Result<JsonListResponse<serde_json::Value>> {
    // let mut acq = db.acquire().await?;

    // let found = PostModel::find_by_blog_id(BlogId::from(0), &mut *acq).await?;

    // TODO: Default values - used as reference when plugin is installed
    Ok(Json(WrappingResponse::okay(ListResponse::all(vec![
        serde_json::json!({
            "_id": 0,
            "_owner": 0,
            "_createdAt": "2024-09-14T03:19:56.4945168Z",
            "_updatedAt": "2024-09-14T03:19:56.4945168Z",
            "content": r#"{"ops":[{"insert":"Holy Crap!\n"}]}"#,
            "title": "Title #1"
        }),
        serde_json::json!({
            "_id": 1,
            "_owner": 0,
            "_createdAt": "2024-09-14T03:19:56.4945168Z",
            "_updatedAt": "2024-09-14T03:19:56.4945168Z",
            "content": r#"{"ops":[{"insert":"Holy Crap!\n"}]}"#,
            "title": "Title #2"
        }),
        serde_json::json!({
            "_id": 2,
            "_owner": 0,
            "_createdAt": "2024-09-14T03:19:56.4945168Z",
            "_updatedAt": "2024-09-14T03:19:56.4945168Z",
            "content": r#"{"ops":[{"insert":"Holy Crap!\n"}]}"#,
            "title": "Title #3"
        }),
    ]))))
}
