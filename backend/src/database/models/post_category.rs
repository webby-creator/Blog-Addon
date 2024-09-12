use eyre::Result;
use serde::Serialize;
use sqlx::{FromRow, SqliteConnection};

use crate::{BlogId, CategoryId, PostId};

#[derive(FromRow, Serialize)]
pub struct PostCategoryModel {
    pub blog_id: BlogId,

    pub post_id: PostId,
    pub category_id: CategoryId,
}

impl PostCategoryModel {
    pub async fn insert(self, db: &mut SqliteConnection) -> Result<PostCategoryModel> {
        sqlx::query(
            "INSERT INTO post_category (blog_id, post_id, category_id) VALUES ($1, $2, $3)",
        )
        .bind(&self.blog_id)
        .bind(&self.post_id)
        .bind(&self.category_id)
        .execute(db)
        .await?;

        Ok(PostCategoryModel {
            blog_id: self.blog_id,
            post_id: self.post_id,
            category_id: self.category_id,
        })
    }
}
