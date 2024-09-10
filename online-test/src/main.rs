use std::sync::Arc;

use online_test::domain::entity::answer::{
    CompletionAnswer, MultipleSelectionAnswer, SingleSelectionAnswer, StandardSource,
};
use online_test::domain::entity::id::Id;
use online_test::domain::entity::question::{
    CompletionQuestion, MultipleSelectionQuestion, Question, SingleSelectionQuestion,
};
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
        Ok(())
    }

    async fn insert_multiple_selection(
        &self,
        content: String,
        options: Vec<String>,
        answer: Vec<u32>,
    ) -> Result<(), QuestionRepositoryError> {
        Ok(())
    }

    async fn insert_completion(
        &self,
        content: String,
        answer: String,
    ) -> Result<(), QuestionRepositoryError> {
        Ok(())
    }

    async fn select_questions(
        &self,
        select_count: SelectCount,
    ) -> Result<Vec<Question>, QuestionRepositoryError> {
        Ok(vec![
            SingleSelectionQuestion::try_new(
                0.into(),
                "Which command is used to trace the system calls made by a process, and which options would you use to trace a specific process ID (PID) and output the results to a file?".into(),
                vec![
                    "<code>strace -p PID -o output.txt</code>".into(),
                    "<code>strace -c -p PID > output.txt</code>".into(),
                    "<code>strace -f -p PID | tee output.txt</code>".into(),
                    "<code>strace -t -p PID > output.txt</code>".into(),
                ],
                SingleSelectionAnswer::<StandardSource>::try_new(0).unwrap(),
            )
            .unwrap()
            .into(),
            MultipleSelectionQuestion::try_new(
                2.into(),
                "In Linux, how can you check the IP address of network interfaces?".into(),
                vec![
                    "<code>ifconfig</code>".into(),
                    "<code>ip addr show</code>".into(),
                    "<code>netstat</code>".into(),
                    "<code>ping</code>".into(),
                ],
                MultipleSelectionAnswer::<StandardSource>::try_new(vec![2, 3]).unwrap(),
            )
            .unwrap()
            .into(),
            CompletionQuestion::try_new(
                3.into(),
                "In Linux, which commands can be used to find files or directories?".into(),
                CompletionAnswer::<StandardSource>::try_new("answer").unwrap(),
            )
            .unwrap()
            .into(),
        ])
    }

    async fn select_questions_by_id(
        &self,
        id: Vec<Id>,
    ) -> Result<Vec<Question>, QuestionRepositoryError> {
        Ok(vec![
            SingleSelectionQuestion::try_new(
                0.into(),
                "Which command is used to trace the system calls made by a process, and which options would you use to trace a specific process ID (PID) and output the results to a file?".into(),
                vec![
                    "<code>strace -p PID -o output.txt</code>".into(),
                    "<code>strace -c -p PID > output.txt</code>".into(),
                    "<code>strace -f -p PID | tee output.txt</code>".into(),
                    "<code>strace -t -p PID > output.txt</code>".into(),
                ],
                SingleSelectionAnswer::<StandardSource>::try_new(0).unwrap(),
            )
            .unwrap()
            .into(),
            MultipleSelectionQuestion::try_new(
                2.into(),
                "In Linux, how can you check the IP address of network interfaces?".into(),
                vec![
                    "<code>ifconfig</code>".into(),
                    "<code>ip addr show</code>".into(),
                    "<code>netstat</code>".into(),
                    "<code>ping</code>".into(),
                ],
                MultipleSelectionAnswer::<StandardSource>::try_new(vec![2, 3]).unwrap(),
            )
            .unwrap()
            .into(),
            CompletionQuestion::try_new(
                3.into(),
                "In Linux, which commands can be used to find files or directories?".into(),
                CompletionAnswer::<StandardSource>::try_new("answer").unwrap(),
            )
            .unwrap()
            .into(),
        ])
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
        Ok(())
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
