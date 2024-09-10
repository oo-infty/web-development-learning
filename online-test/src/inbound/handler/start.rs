use std::sync::Arc;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use snafu::ResultExt;

use crate::domain::application::{Core, CoreError};
use crate::domain::entity::test::Test;
use crate::inbound::error::{ApiError, NotLoggedInSnafu, UnknownSnafu};

#[derive(Debug, Deserialize)]
pub struct StartRequest {
    login_id: usize,
}

#[derive(Debug, Serialize)]
pub struct StartResponse {
    #[serde(flatten)]
    test: Test,
}

pub async fn handle_start(
    State(core): State<Arc<Core>>,
    Json(request): Json<StartRequest>,
) -> Result<Json<StartResponse>, ApiError> {
    let res = core.start(request.login_id.into()).await;

    if let Err(err) = res {
        match err {
            CoreError::SessionNotFound { .. } => NotLoggedInSnafu.fail(),
            _ => Err(err.into()).context(UnknownSnafu),
        }
    } else {
        Ok(StartResponse { test: res.unwrap() }.into())
    }
}
