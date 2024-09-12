use std::sync::Arc;

use online_test::repository::connection::{AsyncSqliteConnectionManager, AsyncSqlitePool};
use online_test::repository::question::QuestionSqliteRepository;
use online_test::repository::score::ScoreSqliteRepository;
use snafu::{prelude::*, Whatever};

use online_test::domain::application::Core;
use online_test::inbound::server::Server;

#[tokio::main(flavor = "current_thread")]
#[snafu::report]
async fn main() -> Result<(), Whatever> {
    let config = AsyncSqliteConnectionManager::new("production/data.db");
    let database_pool: Arc<AsyncSqlitePool> = AsyncSqlitePool::builder(config)
        .build()
        .whatever_context("Could not initialize database pool")?
        .into();

    let question_repository = Arc::new(QuestionSqliteRepository::new(Arc::clone(&database_pool)));
    let score_repository = Arc::new(ScoreSqliteRepository::new(Arc::clone(&database_pool)));

    let core = Arc::new(Core::new(question_repository, score_repository));

    let addr = std::env::var("SERVER_ADDR").unwrap_or("0.0.0.0:8080".to_owned());
    Server::new(addr.parse().unwrap(), core)
        .await
        .unwrap()
        .serve()
        .await
        .whatever_context("Server error occurred")?;

    Ok(())
}
