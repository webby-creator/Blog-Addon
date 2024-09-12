use eyre::Result;
use serde::Serialize;
use sqlx::{FromRow, SqliteConnection};

use crate::{BlogId, PostId, TagId};

#[derive(FromRow, Serialize)]
pub struct PostTagModel {
    pub blog_id: BlogId,

    pub post_id: PostId,
    pub tag_id: TagId,
}

impl PostTagModel {
    pub async fn insert(self, db: &mut SqliteConnection) -> Result<PostTagModel> {
        sqlx::query("INSERT INTO post_tag (blog_id, post_id, tag_id) VALUES ($1, $2, $3)")
            .bind(&self.blog_id)
            .bind(&self.post_id)
            .bind(&self.tag_id)
            .execute(db)
            .await?;

        Ok(PostTagModel {
            blog_id: self.blog_id,
            post_id: self.post_id,
            tag_id: self.tag_id,
        })
    }
}
