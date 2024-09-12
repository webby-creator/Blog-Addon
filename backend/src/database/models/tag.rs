use eyre::Result;
use serde::Serialize;
use sqlx::{FromRow, SqliteConnection};
use uuid::Uuid;

use crate::TagId;

pub struct NewTagModel {
    pub name: String,
}

#[derive(FromRow, Serialize)]
pub struct TagModel {
    pub id: TagId,
    pub name: String,
}

impl NewTagModel {
    pub async fn insert(self, db: &mut SqliteConnection) -> Result<TagModel> {
        let resp = sqlx::query("INSERT INTO tag (name) VALUES ($1)")
            .bind(&self.name)
            .execute(db)
            .await?;

        Ok(TagModel {
            id: TagId::from(resp.last_insert_rowid()),
            name: self.name,
        })
    }
}

impl TagModel {
    pub async fn find_one_by_guid(guid: Uuid, db: &mut SqliteConnection) -> Result<Option<Self>> {
        Ok(sqlx::query_as("SELECT id, name FROM tag WHERE guid = $1")
            .bind(guid)
            .fetch_optional(db)
            .await?)
    }

    pub async fn find_all(db: &mut SqliteConnection) -> Result<Vec<Self>> {
        Ok(sqlx::query_as("SELECT id, name FROM tag")
            .fetch_all(db)
            .await?)
    }
}
