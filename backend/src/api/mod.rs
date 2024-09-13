use std::net::SocketAddr;

use axum::{Extension, Router};
use eyre::Result;
use sqlx::SqlitePool;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

use crate::upload::register_b2;

mod blog;
mod register;

pub async fn serve(pool: SqlitePool) -> Result<()> {
    let port = 5940;

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    debug!("addons listening on {addr}");

    let uploader = register_b2().await;

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(
        listener,
        Router::new()
            .nest("/registration", register::routes())
            .nest("/blog", blog::routes())
            .layer(TraceLayer::new_for_http())
            .layer(Extension(uploader))
            .with_state(pool),
    )
    .await?;

    Ok(())
}
