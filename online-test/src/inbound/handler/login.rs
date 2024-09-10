use std::sync::Arc;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use snafu::ResultExt;

use crate::domain::application::Core;
use crate::domain::entity::user::User;
use crate::inbound::error::{ApiError, DataInvalidSnafu, UnknownSnafu};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    username: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    login_id: usize,
}

pub async fn handle_login(
    State(core): State<Arc<Core>>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, ApiError> {
    let username = request.username;
    let user = User::try_new(username)
        .map_err(Into::into)
        .context(DataInvalidSnafu)?;

    let login_id = core
        .login(user)
        .await
        .map_err(Into::into)
        .context(UnknownSnafu)?;

    let response = LoginResponse {
        login_id: login_id.inner(),
    }
    .into();

    Ok(response)
}
