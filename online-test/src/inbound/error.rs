use std::error::Error;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use snafu::{prelude::*, Report};

use crate::domain::entity::user::User;

#[derive(Debug, Snafu)]
#[non_exhaustive]
#[snafu(visibility(pub(super)))]
pub enum ApiError {
    #[snafu(display("Received invalid data"))]
    DataInvalid { source: Box<dyn Error> },
    #[snafu(display("Could not serve without logging in"))]
    NotLoggedIn,
    #[snafu(display("Test is not authencated by system or expired"))]
    TestInvalidOrExpired,
    #[snafu(display("Could not find information for user {user}"))]
    UserNotFound { user: User },
    #[snafu(display("Unknown error occurred"))]
    Unknown { source: Box<dyn Error> },
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let report = Report::from_error(self);
        (StatusCode::INTERNAL_SERVER_ERROR, report.to_string()).into_response()
    }
}
