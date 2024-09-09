use std::sync::Arc;

use online_test::domain::entity::id::Id;
use online_test::domain::entity::question::Question;
use online_test::domain::entity::score::Score;
use online_test::domain::entity::user::User;
use online_test::domain::repository::question::{
    QuestionRepository, QuestionRepositoryError, SelectCount,
};
use online_test::domain::repository::score::{Record, ScoreRepository, ScoreRepositoryError};
use snafu::{prelude::*, ResultExt, Whatever};
use tokio::time::Duration;

use online_test::domain::application::Core;
use online_test::inbound::server::Server;
use tokio::time::Instant;

#[tokio::main]
#[snafu::report]
async fn main() -> Result<(), Whatever> {
    let core = Arc::new(Core::new(Arc::new(Qr {}), Arc::new(Sr {})));

    Server::new("127.0.0.1:8080".parse().unwrap(), core)
        .await
        .unwrap()
        .serve()
        .await
        .whatever_context("Server error occurred")?;

    Ok(())
}

#[derive(Debug)]
struct Qr {}

#[async_trait::async_trait]
impl QuestionRepository for Qr {
    async fn insert_single_selection(
        &self,
        content: String,
        options: Vec<String>,
        answer: u32,
    ) -> Result<(), QuestionRepositoryError> {
        todo!()
    }

    async fn insert_multiple_selection(
        &self,
        content: String,
        options: Vec<String>,
        answer: Vec<u32>,
    ) -> Result<(), QuestionRepositoryError> {
        todo!()
    }

    async fn insert_completion(
        &self,
        content: String,
        answer: String,
    ) -> Result<(), QuestionRepositoryError> {
        todo!()
    }

    async fn select_questions(
        &self,
        select_count: SelectCount,
    ) -> Result<Vec<Question>, QuestionRepositoryError> {
        todo!()
    }

    async fn select_questions_by_id(
        &self,
        id: Vec<Id>,
    ) -> Result<Vec<Question>, QuestionRepositoryError> {
        todo!()
    }
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
        todo!()
    }

    async fn query_all_sorted(&self, user: &User) -> Result<Vec<Record>, ScoreRepositoryError> {
        todo!()
    }

    async fn query_best(&self, user: &User) -> Result<Record, ScoreRepositoryError> {
        todo!()
    }

    async fn query_latest(&self, user: &User) -> Result<Record, ScoreRepositoryError> {
        todo!()
    }
}
