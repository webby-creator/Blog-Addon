use eyre::Result;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::Serialize;
use sqlx::{FromRow, SqliteConnection};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{BlogId, PostId};

pub struct NewPostModel {
    pub blog_id: BlogId,

    pub title: String,
    pub content: String,
    pub slug: String,

    pub status: PostStatus,
}

#[derive(FromRow, Serialize)]
pub struct PostModel {
    pub id: PostId,

    pub blog_id: BlogId,

    pub title: String,
    pub content: String,
    pub slug: String,

    pub status: PostStatus,

    pub delete_reason: Option<String>,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}

impl NewPostModel {
    pub async fn insert(self, db: &mut SqliteConnection) -> Result<PostModel> {
        let now = OffsetDateTime::now_utc();

        let resp = sqlx::query(
            "INSERT INTO post (blog_id, title, content, slug, status, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $6)",
        )
        .bind(self.blog_id)
        .bind(&self.title)
        .bind(&self.content)
        .bind(&self.slug)
        .bind(&self.status)
        .bind(now)
        .execute(db)
        .await?;

        Ok(PostModel {
            id: PostId::from(resp.last_insert_rowid()),
            blog_id: self.blog_id,
            title: self.title,
            content: self.content,
            slug: self.slug,
            status: self.status,
            delete_reason: None,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        })
    }
}

impl PostModel {
    pub async fn find_one_by_guid(guid: Uuid, db: &mut SqliteConnection) -> Result<Option<Self>> {
        Ok(sqlx::query_as(
            "SELECT id, blog_id, title, content, slug, status, delete_reason, created_at, updated_at, deleted_at FROM post WHERE guid = $1"
        )
        .bind(guid)
        .fetch_optional(db)
        .await?)
    }

    pub async fn find_all(db: &mut SqliteConnection) -> Result<Vec<Self>> {
        Ok(sqlx::query_as(
            "SELECT id, blog_id, title, content, slug, status, delete_reason, created_at, updated_at, deleted_at FROM post"
        )
        .fetch_all(db)
        .await?)
    }

    pub async fn delete(
        id: PostId,
        reason: Option<String>,
        db: &mut SqliteConnection,
    ) -> Result<u64> {
        let res = sqlx::query("UPDATE post SET deleted_at = $2, delete_reason = $3 WHERE id = $1")
            .bind(id)
            .bind(OffsetDateTime::now_utc())
            .bind(reason)
            .execute(db)
            .await?;

        Ok(res.rows_affected())
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum PostStatus {
    Draft = 0,
    Published = 1,
}

impl FromRow<'_, ::sqlx::sqlite::SqliteRow> for PostStatus {
    fn from_row(row: &::sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        use ::sqlx::Row;

        Ok(Self::try_from(row.try_get::<i32, _>(0)? as u8).unwrap())
    }
}

impl ::sqlx::Encode<'_, ::sqlx::sqlite::Sqlite> for PostStatus {
    fn encode_by_ref(
        &self,
        buf: &mut <::sqlx::sqlite::Sqlite as sqlx::Database>::ArgumentBuffer<'_>,
    ) -> std::result::Result<::sqlx::encode::IsNull, ::sqlx::error::BoxDynError> {
        ::sqlx::Encode::<::sqlx::sqlite::Sqlite>::encode_by_ref(&(*self as u8 as i32), buf)
    }
}

impl ::sqlx::Decode<'_, ::sqlx::sqlite::Sqlite> for PostStatus {
    fn decode(
        value: <::sqlx::sqlite::Sqlite as sqlx::Database>::ValueRef<'_>,
    ) -> std::result::Result<Self, ::sqlx::error::BoxDynError> {
        Ok(Self::try_from(
            <i32 as ::sqlx::Decode<::sqlx::sqlite::Sqlite>>::decode(value)? as u8,
        )?)
    }
}

impl ::sqlx::Type<::sqlx::sqlite::Sqlite> for PostStatus {
    fn type_info() -> ::sqlx::sqlite::SqliteTypeInfo {
        <i32 as ::sqlx::Type<::sqlx::sqlite::Sqlite>>::type_info()
    }
}
