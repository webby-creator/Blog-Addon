use eyre::Result;
use serde::Serialize;
use sqlx::{FromRow, SqliteConnection};
use uuid::Uuid;

use crate::CategoryId;

pub struct NewCategoryModel {
    pub name: String,
}

#[derive(FromRow, Serialize)]
pub struct CategoryModel {
    pub id: CategoryId,
    pub name: String,
}

impl NewCategoryModel {
    pub async fn insert(self, db: &mut SqliteConnection) -> Result<CategoryModel> {
        let resp = sqlx::query("INSERT INTO category (name) VALUES ($1)")
            .bind(&self.name)
            .execute(db)
            .await?;

        Ok(CategoryModel {
            id: CategoryId::from(resp.last_insert_rowid()),
            name: self.name,
        })
    }
}

impl CategoryModel {
    pub async fn find_one_by_guid(guid: Uuid, db: &mut SqliteConnection) -> Result<Option<Self>> {
        Ok(
            sqlx::query_as("SELECT id, name FROM category WHERE guid = $1")
                .bind(guid)
                .fetch_optional(db)
                .await?,
        )
    }

    pub async fn find_all(db: &mut SqliteConnection) -> Result<Vec<Self>> {
        Ok(sqlx::query_as("SELECT id, name FROM category")
            .fetch_all(db)
            .await?)
    }
}
