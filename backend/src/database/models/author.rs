use eyre::Result;
use serde::Serialize;
use sqlx::{FromRow, SqliteConnection};
use uuid::Uuid;

use crate::{AuthorId, MemberUuid, WebsiteUuid};

pub struct NewAuthorModel {
    pub blog_id: WebsiteUuid,
    pub external_member_id: MemberUuid,

    pub name: String,
    pub email: Option<String>,
}

#[derive(FromRow, Serialize)]
pub struct AuthorModel {
    pub id: AuthorId,

    pub blog_id: WebsiteUuid,
    pub external_member_id: MemberUuid,

    pub name: String,
    pub email: Option<String>,
}

impl NewAuthorModel {
    pub async fn insert(self, db: &mut SqliteConnection) -> Result<AuthorModel> {
        let resp = sqlx::query(
            "INSERT INTO author (blog_id, external_member_id, name, email) VALUES ($1, $2, $3, $4)",
        )
        .bind(self.blog_id)
        .bind(self.external_member_id)
        .bind(&self.name)
        .bind(&self.email)
        .execute(db)
        .await?;

        Ok(AuthorModel {
            id: AuthorId::from(resp.last_insert_rowid() as i32),
            blog_id: self.blog_id,
            external_member_id: self.external_member_id,
            name: self.name,
            email: None,
        })
    }
}

impl AuthorModel {
    pub async fn find_one_by_guid(guid: Uuid, db: &mut SqliteConnection) -> Result<Option<Self>> {
        Ok(sqlx::query_as(
            "SELECT id, blog_id, external_member_id, name, email FROM author WHERE guid = $1",
        )
        .bind(guid)
        .fetch_optional(db)
        .await?)
    }

    pub async fn find_all(db: &mut SqliteConnection) -> Result<Vec<Self>> {
        Ok(
            sqlx::query_as("SELECT id, blog_id, external_member_id, name, email FROM author")
                .fetch_all(db)
                .await?,
        )
    }

    pub async fn delete(id: AuthorId, db: &mut SqliteConnection) -> Result<u64> {
        let res = sqlx::query("DELETE FROM author WHERE id = $1")
            .bind(id)
            .execute(db)
            .await?;

        Ok(res.rows_affected())
    }
}
