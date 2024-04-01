pub mod routes;
pub mod templates;
pub mod url_service;

use std::sync::{Arc, Mutex};

use axum::{routing::get, Router};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{fmt::time, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    routes::{home_page, redirect, shorten_url},
    url_service::UrlService,
};

#[tokio::main]

async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().with_timer(time::LocalTime::rfc_3339()))
        .init();
    info!("initializing router!");

    let url_service = Arc::new(Mutex::new(UrlService::new()));

    let app = Router::new()
        .route("/:key_id", get(redirect))
        .route("/", get(home_page).post(shorten_url))
        .with_state(url_service)
        .layer(TraceLayer::new_for_http());

    let port = 8080_u16;
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    info!("router initialized, now listening on port {}", port);
    axum::serve(listener, app).await?;

    Ok(())
}
