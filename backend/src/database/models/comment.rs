use webby_addon_common::MemberUuid;
use eyre::Result;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::Serialize;
use sqlx::{FromRow, SqliteConnection};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{BlogId, CommentId, PostId};

pub struct NewCommentModel {
    pub blog_id: BlogId,
    pub post_id: PostId,

    pub external_member_id: Option<MemberUuid>,

    pub author_name: String,
    pub email: Option<String>,
    pub comment: String,

    pub status: CommentStatus,
}

#[derive(FromRow, Serialize)]
pub struct CommentModel {
    pub id: CommentId,

    pub blog_id: BlogId,
    pub post_id: PostId,

    pub external_member_id: Option<MemberUuid>,

    pub author_name: String,
    pub email: Option<String>,
    pub comment: String,

    pub status: CommentStatus,

    pub delete_reason: Option<String>,
    pub deleted_at: Option<OffsetDateTime>,
}

impl NewCommentModel {
    pub async fn insert(self, db: &mut SqliteConnection) -> Result<CommentModel> {
        let now = OffsetDateTime::now_utc();

        let resp = sqlx::query(
            "INSERT INTO comment (blog_id, post_id, external_member_id, author_name, email, comment, status, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $8)",
        )
            .bind(self.blog_id)
            .bind(self.post_id)
            .bind(self.external_member_id)
            .bind(&self.author_name)
            .bind(&self.email)
            .bind(&self.comment)
            .bind(self.status)
            .bind(now)
            .execute(db)
            .await?;

        Ok(CommentModel {
            id: CommentId::from(resp.last_insert_rowid()),
            blog_id: self.blog_id,
            post_id: self.post_id,
            external_member_id: self.external_member_id,
            author_name: self.author_name,
            email: self.email,
            comment: self.comment,
            status: self.status,
            delete_reason: None,
            deleted_at: None,
        })
    }
}

impl CommentModel {
    pub async fn find_one_by_guid(guid: Uuid, db: &mut SqliteConnection) -> Result<Option<Self>> {
        Ok(sqlx::query_as(
            "SELECT id, blog_id, post_id, external_member_id, author_name, email, comment, status, delete_reason, deleted_at FROM comment WHERE guid = $1"
        )
        .bind(guid)
        .fetch_optional(db)
        .await?)
    }

    pub async fn find_all(db: &mut SqliteConnection) -> Result<Vec<Self>> {
        Ok(sqlx::query_as(
            "SELECT id, blog_id, post_id, external_member_id, author_name, email, comment, status, delete_reason, deleted_at FROM comment"
        )
        .fetch_all(db)
        .await?)
    }

    pub async fn delete(
        id: CommentId,
        reason: Option<String>,
        db: &mut SqliteConnection,
    ) -> Result<u64> {
        let res =
            sqlx::query("UPDATE comment SET deleted_at = $2, delete_reason = $3 WHERE id = $1")
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
pub enum CommentStatus {
    Pending = 0,
    Approved = 1,
    Denied = 2,
}

impl FromRow<'_, ::sqlx::sqlite::SqliteRow> for CommentStatus {
    fn from_row(row: &::sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        use ::sqlx::Row;

        Ok(Self::try_from(row.try_get::<i32, _>(0)? as u8).unwrap())
    }
}

impl ::sqlx::Encode<'_, ::sqlx::sqlite::Sqlite> for CommentStatus {
    fn encode_by_ref(
        &self,
        buf: &mut <::sqlx::sqlite::Sqlite as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        ::sqlx::Encode::<::sqlx::sqlite::Sqlite>::encode_by_ref(&(*self as u8 as i32), buf)
    }
}

impl ::sqlx::Decode<'_, ::sqlx::sqlite::Sqlite> for CommentStatus {
    fn decode(
        value: <::sqlx::sqlite::Sqlite as sqlx::database::HasValueRef<'_>>::ValueRef,
    ) -> std::result::Result<Self, sqlx::error::BoxDynError> {
        Ok(Self::try_from(
            <i32 as ::sqlx::Decode<::sqlx::sqlite::Sqlite>>::decode(value)? as u8,
        )?)
    }
}

impl ::sqlx::Type<::sqlx::sqlite::Sqlite> for CommentStatus {
    fn type_info() -> ::sqlx::sqlite::SqliteTypeInfo {
        <i32 as ::sqlx::Type<::sqlx::sqlite::Sqlite>>::type_info()
    }
}
