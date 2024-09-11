use std::error::Error;
use std::fmt::Debug;

use async_trait::async_trait;
use snafu::prelude::*;

use crate::domain::entity::answer::TryNewAnswerError;
use crate::domain::entity::id::Id;
use crate::domain::entity::question::{Question, TryNewQuestionError};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait QuestionRepository: Debug + Send + Sync + 'static {
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

    async fn select_questions_by_id(
        &self,
        id: Vec<Id>,
    ) -> Result<Vec<Question>, QuestionRepositoryError>;
}

#[derive(Debug, Snafu)]
#[non_exhaustive]
#[snafu(visibility(pub(crate)))]
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
    #[snafu(display("Could not find question with ID {id}"))]
    NotFound { id: Id },
    #[snafu(whatever, display("Unknown error: {message}"))]
    Unknown {
        message: String,
        #[snafu(source(from(Box<dyn Error + Send>, Some)))]
        source: Option<Box<dyn Error + Send>>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectCount {
    pub single_selection: usize,
    pub multiple_selection: usize,
    pub completion: usize,
}
