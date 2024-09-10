use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
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
            .route("/api/login", post(super::handler::login::handle_login))
            .route("/api/start", post(super::handler::start::handle_start))
            .route("/api/submit", post(super::handler::submit::handle_submit))
            .with_state(self.core);

        axum::serve(self.listener, router.into_make_service())
            .await
            .whatever_context("Server error")
    }
}
