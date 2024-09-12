use std::error::Error;
use std::fmt::Debug;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use snafu::prelude::*;
use tokio::time::Duration;

use crate::domain::entity::score::Score;
use crate::domain::entity::user::User;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait ScoreRepository: Debug + Send + Sync + 'static {
    async fn insert(
        &self,
        user: User,
        score: Score,
        end_time: DateTime<Utc>,
        duration: Duration,
    ) -> Result<(), ScoreRepositoryError>;

    async fn query_all_sorted(&self, user: &User) -> Result<Vec<Record>, ScoreRepositoryError>;

    async fn query_best(&self, user: &User) -> Result<Record, ScoreRepositoryError>;

    async fn query_latest(&self, user: &User) -> Result<Record, ScoreRepositoryError>;
}

#[derive(Debug, Snafu)]
#[non_exhaustive]
#[snafu(visibility(pub(crate)))]
pub enum ScoreRepositoryError {
    #[snafu(display("Could not find username {}", user.inner()))]
    NotFound { user: User },
    #[snafu(whatever, display("Unknown error: {message}"))]
    Unknown {
        message: String,
        #[snafu(source(from(Box<dyn Error + Send>, Some)))]
        source: Option<Box<dyn Error + Send>>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Record {
    pub score: Score,
    pub end_time: DateTime<Utc>,
    pub duration: Duration,
}
