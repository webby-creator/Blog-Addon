use addon_common::{MemberUuid, WebsiteUuid};
use eyre::Result;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::Serialize;
use sqlx::{FromRow, SqliteConnection};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::BlogId;

pub struct NewBlogModel {
    pub instance_id: Uuid,

    pub external_website_id: WebsiteUuid,
    pub external_member_id: MemberUuid,

    pub name: String,
}

#[derive(FromRow, Serialize)]
pub struct BlogModel {
    pub id: BlogId,

    pub instance_id: Uuid,

    pub external_website_id: WebsiteUuid,
    pub external_member_id: MemberUuid,

    pub name: String,

    pub setup_position: SetupPosition,

    pub delete_reason: Option<String>,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}

impl NewBlogModel {
    pub async fn insert(self, db: &mut SqliteConnection) -> Result<BlogModel> {
        let now = OffsetDateTime::now_utc();
        let setup_position = SetupPosition::None;

        let resp = sqlx::query(
            "INSERT INTO blog (instance_id, external_website_id, external_member_id, name, setup_position, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $6)",
        )
        .bind(self.instance_id)
        .bind(self.external_website_id)
        .bind(self.external_member_id)
        .bind(&self.name)
        .bind(setup_position)
        .bind(now)
        .execute(db)
        .await?;

        Ok(BlogModel {
            id: BlogId::from(resp.last_insert_rowid() as i32),
            instance_id: self.instance_id,
            external_website_id: self.external_website_id,
            external_member_id: self.external_member_id,
            name: self.name,
            setup_position,
            delete_reason: None,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        })
    }
}

impl BlogModel {
    pub async fn find_one_by_guid(guid: Uuid, db: &mut SqliteConnection) -> Result<Option<Self>> {
        Ok(sqlx::query_as(
            "SELECT id, instance_id, external_website_id, external_member_id, name, setup_position, delete_reason, created_at, updated_at, deleted_at FROM blog WHERE guid = $1"
        )
        .bind(guid)
        .fetch_optional(db)
        .await?)
    }

    pub async fn find_all(db: &mut SqliteConnection) -> Result<Vec<Self>> {
        Ok(sqlx::query_as(
            "SELECT id, instance_id, external_website_id, external_member_id, name, setup_position, delete_reason, created_at, updated_at, deleted_at FROM blog"
        )
        .fetch_all(db)
        .await?)
    }

    pub async fn delete(
        id: BlogId,
        reason: Option<String>,
        db: &mut SqliteConnection,
    ) -> Result<u64> {
        let res = sqlx::query("UPDATE blog SET deleted_at = $2, delete_reason = $3 WHERE id = $1")
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
pub enum SetupPosition {
    Done = 0,
    None = 1,
}

impl FromRow<'_, ::sqlx::sqlite::SqliteRow> for SetupPosition {
    fn from_row(row: &::sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        use ::sqlx::Row;

        Ok(Self::try_from(row.try_get::<i32, _>(0)? as u8).unwrap())
    }
}

impl ::sqlx::Encode<'_, ::sqlx::sqlite::Sqlite> for SetupPosition {
    fn encode_by_ref(
        &self,
        buf: &mut <::sqlx::sqlite::Sqlite as sqlx::Database>::ArgumentBuffer<'_>,
    ) -> std::result::Result<::sqlx::encode::IsNull, ::sqlx::error::BoxDynError> {
        ::sqlx::Encode::<::sqlx::sqlite::Sqlite>::encode_by_ref(&(*self as u8 as i32), buf)
    }
}

impl ::sqlx::Decode<'_, ::sqlx::sqlite::Sqlite> for SetupPosition {
    fn decode(
        value: <::sqlx::sqlite::Sqlite as sqlx::Database>::ValueRef<'_>,
    ) -> std::result::Result<Self, ::sqlx::error::BoxDynError> {
        Ok(Self::try_from(
            <i32 as ::sqlx::Decode<::sqlx::sqlite::Sqlite>>::decode(value)? as u8,
        )?)
    }
}

impl ::sqlx::Type<::sqlx::sqlite::Sqlite> for SetupPosition {
    fn type_info() -> ::sqlx::sqlite::SqliteTypeInfo {
        <i32 as ::sqlx::Type<::sqlx::sqlite::Sqlite>>::type_info()
    }
}
