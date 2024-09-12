#[macro_use]
extern crate tracing;

use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod api;
mod common;
mod database;
mod error;
mod upload;

pub use database::id::*;
pub use database::models;
pub use error::{Error, Result};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "blog_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // if tokio::fs::metadata(database::DATABASE_PATH).await.is_ok() {
    //     tokio::fs::remove_file(database::DATABASE_PATH).await?;
    // }

    let (_is_new, pool) = database::init().await?;

    Ok(api::serve(pool).await?)
}
