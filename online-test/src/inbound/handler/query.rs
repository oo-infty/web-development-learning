use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};
use snafu::prelude::*;

use crate::domain::application::{Core, CoreError};
use crate::domain::repository::score::Record;
use crate::domain::session::login::QueryKind;
use crate::inbound::error::{ApiError, NotLoggedInSnafu, UnknownSnafu};

#[derive(Debug, Deserialize)]
pub struct QueryRequest {
    login_id: usize,
    kind: String,
}

#[derive(Debug, Serialize)]
pub struct QueryResponse {
    result: Vec<ResultData>,
}

#[derive(Debug, Serialize)]
pub struct ResultData {
    score: f32,
    end_time: String,
    duration: usize,
}

impl From<Record> for ResultData {
    fn from(value: Record) -> Self {
        Self {
            score: value.score.inner(),
            end_time: value.end_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            duration: value.duration.as_secs() as usize,
        }
    }
}

pub async fn handle_query(
    State(core): State<Arc<Core>>,
    Json(request): Json<QueryRequest>,
) -> Result<Json<QueryResponse>, ApiError> {
    let login_id = request.login_id;

    let res = if request.kind == "all" {
        core.query_all(login_id.into()).await
    } else if request.kind == "latest" {
        core.query(login_id.into(), QueryKind::Latest)
            .await
            .map(|r| vec![r])
    } else {
        core.query(login_id.into(), QueryKind::Best)
            .await
            .map(|r| vec![r])
    };

    if let Err(err) = res {
        match err {
            CoreError::SessionNotFound { .. } => NotLoggedInSnafu.fail(),
            _ => Err(err.into()).context(UnknownSnafu),
        }
    } else {
        let response = QueryResponse {
            result: res.unwrap().into_iter().map(Into::into).collect(),
        };

        Ok(response.into())
    }
}
