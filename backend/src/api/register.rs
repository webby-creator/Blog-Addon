use axum::{extract, routing::post, Json, Router};
use sqlx::SqlitePool;

use crate::{
    common::{MemberPartial, WebsitePartial},
    models::{BlogModel, NewBlogModel},
    MemberUuid, Result, WebsiteUuid,
};

use super::{JsonResponse, WrappingResponse};

pub fn routes() -> Router<SqlitePool> {
    Router::new().route("/", post(post_install))
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct RegisterNew {
    website_id: WebsiteUuid,
    owner_id: MemberUuid,

    member: MemberPartial,
    website: WebsitePartial,
}

async fn post_install(
    extract::State(db): extract::State<SqlitePool>,
    extract::Json(value): extract::Json<RegisterNew>,
) -> Result<JsonResponse<serde_json::Value>> {
    let mut _acq = db.acquire().await?;

    // let _model = NewBlogModel {
    //     external_website_id: value.website_id,
    //     external_member_id: value.owner_id,
    //     name: value.website.name,
    // }
    // .insert(&mut *acq)
    // .await?;

    Ok(Json(WrappingResponse::okay(serde_json::json!({
        //
    }))))
}