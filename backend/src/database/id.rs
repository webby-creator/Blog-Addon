use std::{
    fmt::{self, Display},
    num::ParseIntError,
    ops::Deref,
    str::FromStr,
};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
// use uuid::{Error as UuidError, Uuid};

macro_rules! create_id {
    ($name:ident, $type_of:ident, $err:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, sqlx::Type)]
        #[sqlx(transparent)]
        #[repr(transparent)]
        pub struct $name($type_of);

        impl $name {
            pub fn none() -> Self {
                Self($type_of::default())
            }

            pub fn is_none(self) -> bool {
                self.0 == Self::none().0
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                Ok(Self($type_of::deserialize(deserializer)?))
            }
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                $type_of::serialize(&self.0, serializer)
            }
        }

        impl Deref for $name {
            type Target = $type_of;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                $type_of::fmt(&self.0, f)
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::none()
            }
        }

        impl PartialEq<$type_of> for $name {
            fn eq(&self, other: &$type_of) -> bool {
                self.0 == *other
            }
        }

        impl From<$type_of> for $name {
            fn from(value: $type_of) -> Self {
                Self(value)
            }
        }

        impl FromStr for $name {
            type Err = $err;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                $type_of::from_str(s).map(Self)
            }
        }
    };
}

create_id!(BlogId, i32, ParseIntError);
create_id!(AuthorId, i32, ParseIntError);

// i64's
create_id!(PostId, i64, ParseIntError);
create_id!(CommentId, i64, ParseIntError);
create_id!(TagId, i64, ParseIntError);
create_id!(CategoryId, i64, ParseIntError);
create_id!(PostTagId, i64, ParseIntError);
create_id!(PostCategoryId, i64, ParseIntError);
