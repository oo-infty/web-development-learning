use std::sync::Arc;

use chrono::Utc;
use online_test::domain::entity::score::Score;
use online_test::domain::entity::user::User;
use online_test::domain::repository::score::{Record, ScoreRepository, ScoreRepositoryError};
use online_test::repository::connection::{AsyncSqliteConnectionManager, AsyncSqlitePool};
use online_test::repository::question::QuestionSqliteRepository;
use snafu::{prelude::*, Whatever};
use tokio::time::Duration;

use online_test::domain::application::Core;
use online_test::inbound::server::Server;
use tokio::time::Instant;

#[tokio::main]
#[snafu::report]
async fn main() -> Result<(), Whatever> {
    let config = AsyncSqliteConnectionManager::new("production/data.db");
    let database_pool: Arc<AsyncSqlitePool> = AsyncSqlitePool::builder(config)
        .build()
        .whatever_context("Could not initialize database pool")?
        .into();

    let question_repository = Arc::new(QuestionSqliteRepository::new(database_pool));

    let core = Arc::new(Core::new(question_repository, Arc::new(Sr {})));

    Server::new("127.0.0.1:8080".parse().unwrap(), core)
        .await
        .unwrap()
        .serve()
        .await
        .whatever_context("Server error occurred")?;

    Ok(())
}

#[derive(Debug)]
struct Sr {}

#[async_trait::async_trait]
impl ScoreRepository for Sr {
    async fn insert(
        &self,
        user: User,
        score: Score,
        end_time: Instant,
        duration: Duration,
    ) -> Result<(), ScoreRepositoryError> {
        Ok(())
    }

    async fn query_all_sorted(&self, user: &User) -> Result<Vec<Record>, ScoreRepositoryError> {
        Ok(vec![Record {
            score: Score::try_new(100f32).unwrap(),
            end_time: Utc::now(),
            duration: Duration::from_secs(630),
        }])
    }

    async fn query_best(&self, user: &User) -> Result<Record, ScoreRepositoryError> {
        Ok(Record {
            score: Score::try_new(100f32).unwrap(),
            end_time: Utc::now(),
            duration: Duration::from_secs(630),
        })
    }

    async fn query_latest(&self, user: &User) -> Result<Record, ScoreRepositoryError> {
        Ok(Record {
            score: Score::try_new(100f32).unwrap(),
            end_time: Utc::now(),
            duration: Duration::from_secs(630),
        })
    }
}
