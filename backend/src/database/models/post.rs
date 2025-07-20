use eyre::Result;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use regex::Regex;
use serde::Serialize;
use sqlx::{types::Json, FromRow, SqliteConnection};
use time::OffsetDateTime;

use crate::{BlogId, PostId};

pub struct NewPostModel {
    pub blog_id: BlogId,

    pub title: String,
    pub content: serde_json::Value,
    pub slug: Option<String>,

    pub status: PostStatus,

    pub post_date: Option<OffsetDateTime>,
}

#[derive(FromRow, Serialize)]
pub struct PostModel {
    pub id: PostId,

    pub blog_id: BlogId,

    pub title: String,
    pub content: Json<serde_json::Value>,
    pub slug: Option<String>,

    // TODO: PostStatus - SQL erroring
    pub status: i32,

    pub post_date: OffsetDateTime,

    pub delete_reason: Option<String>,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}

impl NewPostModel {
    pub async fn insert(self, db: &mut SqliteConnection) -> Result<PostModel> {
        let now = OffsetDateTime::now_utc();
        let post_date = self.post_date.unwrap_or(now);

        let resp = sqlx::query(
            "INSERT INTO post (blog_id, title, content, slug, status, post_date, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $7)",
        )
        .bind(self.blog_id)
        .bind(&self.title)
        .bind(Json(&self.content))
        .bind(&self.slug)
        .bind(&self.status)
        .bind(post_date)
        .bind(now)
        .execute(db)
        .await?;

        Ok(PostModel {
            id: PostId::from(resp.last_insert_rowid()),
            blog_id: self.blog_id,
            title: self.title,
            content: Json(self.content),
            slug: self.slug,
            status: self.status as u8 as i32,
            post_date,
            delete_reason: None,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        })
    }
}

impl PostModel {
    pub async fn update(&mut self, db: &mut SqliteConnection) -> Result<u64> {
        self.updated_at = OffsetDateTime::now_utc();

        let res =
            sqlx::query("UPDATE post SET title = $2, content = $3, slug = $4, status = $5, post_date = $6, updated_at = $7 WHERE id = $1")
                .bind(self.id)
                .bind(&self.title)
                .bind(&self.content)
                .bind(&self.slug)
                .bind(self.status)
                .bind(&self.post_date)
                .bind(self.updated_at)
                .execute(db)
                .await?;

        Ok(res.rows_affected())
    }

    pub async fn find_one_by_id(id: PostId, db: &mut SqliteConnection) -> Result<Option<Self>> {
        Ok(sqlx::query_as(
            "SELECT id, blog_id, title, content, slug, status, post_date, delete_reason, created_at, updated_at, deleted_at FROM post WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(db)
        .await?)
    }

    pub async fn find_by_blog_id(id: BlogId, db: &mut SqliteConnection) -> Result<Vec<Self>> {
        Ok(sqlx::query_as(
            "SELECT id, blog_id, title, content, slug, status, post_date, delete_reason, created_at, updated_at, deleted_at FROM post WHERE blog_id = $1"
        )
        .bind(id)
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

#[derive(
    Debug, Clone, Copy, serde::Serialize, serde::Deserialize, IntoPrimitive, TryFromPrimitive,
)]
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
        buf: &mut <::sqlx::sqlite::Sqlite as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        ::sqlx::Encode::<::sqlx::sqlite::Sqlite>::encode_by_ref(&(*self as u8 as i32), buf)
    }
}

impl ::sqlx::Decode<'_, ::sqlx::sqlite::Sqlite> for PostStatus {
    fn decode(
        value: <::sqlx::sqlite::Sqlite as sqlx::database::HasValueRef<'_>>::ValueRef,
    ) -> std::result::Result<Self, sqlx::error::BoxDynError> {
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

pub fn slugify(value: &str) -> String {
    // trim leading/trailing white space
    let mut value = Regex::new(r#"^\s+|\s+$"#)
        .unwrap()
        .replace_all(value, "")
        .to_string();

    value = value.to_lowercase();

    // remove any non-alphanumeric characters
    value = Regex::new(r#"[^a-z0-9 -]"#)
        .unwrap()
        .replace(&value, "")
        .to_string();
    // replace spaces with hyphens
    value = Regex::new(r#"\s+"#)
        .unwrap()
        .replace(&value, "-")
        .to_string();
    // remove consecutive hyphens
    value = Regex::new(r#"-+"#)
        .unwrap()
        .replace(&value, "-")
        .to_string();

    if value.len() > 30 {
        value = value.chars().take(30).collect();
    }

    value
}
