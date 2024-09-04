use std::error::Error;

use snafu::prelude::*;
use tokio::time::{Duration, Instant};

use crate::domain::entity::score::Score;
use crate::domain::entity::user::User;

#[async_trait::async_trait]
#[cfg_attr(test, mockall::automock)]
pub trait ScoreRepository: Send + Sync + 'static {
    async fn insert(
        &self,
        user: User,
        score: Score,
        end_time: Instant,
        duration: Duration,
    ) -> Result<(), ScoreRepositoryError>;

    async fn query_all_sorted(&self, user: &User) -> Result<Vec<Record>, ScoreRepositoryError>;

    async fn query_best(&self, user: &User) -> Result<Record, ScoreRepositoryError>;

    async fn query_latest(&self, user: &User) -> Result<Record, ScoreRepositoryError>;
}

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum ScoreRepositoryError {
    #[snafu(display("Could not find username {}", user.inner()))]
    NotFound { user: User },
    #[snafu(whatever, display("Unknown error: {message}"))]
    Unknown {
        message: String,
        #[snafu(source(from(Box<dyn Error>, Some)))]
        source: Option<Box<dyn Error>>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Record {
    pub score: Score,
    pub end_time: Instant,
    pub duration: Duration,
}
