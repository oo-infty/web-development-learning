use std::error::Error;

use snafu::prelude::*;

use crate::domain::entity::answer::TryNewAnswerError;
use crate::domain::entity::question::{Question, TryNewQuestionError};

#[async_trait::async_trait]
#[cfg_attr(test, mockall::automock)]
pub trait QuestionRepository: Send + Sync + 'static {
    async fn insert_single_selection(
        &self,
        content: String,
        options: Vec<String>,
        answer: u32,
    ) -> Result<(), QuestionRepositoryError>;

    async fn insert_multiple_selection(
        &self,
        content: String,
        options: Vec<String>,
        answer: Vec<u32>,
    ) -> Result<(), QuestionRepositoryError>;

    async fn insert_completion(
        &self,
        content: String,
        answer: String,
    ) -> Result<(), QuestionRepositoryError>;

    async fn select_questions(
        &self,
        select_count: SelectCount,
    ) -> Result<Vec<Question>, QuestionRepositoryError>;
}

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum QuestionRepositoryError {
    #[snafu(display("Could not create valid answer"))]
    AnswerInvalid { source: TryNewAnswerError },
    #[snafu(display("Could not create valid question"))]
    QuestionInvalid { source: TryNewQuestionError },
    #[snafu(display("Could not select {expected} {which}(s) from {total}"))]
    Insufficient {
        which: String,
        expected: usize,
        total: usize,
    },
    #[snafu(whatever, display("Unknown error: {message}"))]
    Unknown {
        message: String,
        #[snafu(source(from(Box<dyn Error>, Some)))]
        source: Option<Box<dyn Error>>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectCount {
    single_selection: usize,
    multiple_selection: usize,
    completion: usize,
}
