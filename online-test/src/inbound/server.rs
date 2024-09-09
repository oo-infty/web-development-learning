use std::net::SocketAddr;
use std::sync::Arc;

use axum::{routing::get, Router};
use snafu::{prelude::*, Whatever};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

use crate::domain::application::Core;

#[derive(Debug)]
pub struct Server {
    listener: TcpListener,
    core: Arc<Core>,
}

impl Server {
    pub async fn new(addr: SocketAddr, core: Arc<Core>) -> Result<Self, Whatever> {
        let listener = TcpListener::bind(addr)
            .await
            .whatever_context(format!("Could not bind to {addr}"))?;

        Ok(Self { listener, core })
    }

    pub async fn serve(self) -> Result<(), Whatever> {
        let router = Router::new()
            .nest_service("/", ServeDir::new("static"))
            .route("/api/start", get(super::handler::start::handle_start))
            .with_state(self.core);

        axum::serve(self.listener, router.into_make_service())
            .await
            .whatever_context("Server error")
    }
}
