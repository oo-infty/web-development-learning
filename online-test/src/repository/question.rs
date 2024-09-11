use std::error::Error;
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::sync::Arc;

use diesel::prelude::*;
use diesel::sql_types::Integer;
use diesel_async::RunQueryDsl;
use snafu::prelude::*;

use crate::domain::entity::answer::{
    CompletionAnswer, MultipleSelectionAnswer, SingleSelectionAnswer, StandardSource,
};
use crate::domain::entity::question::{
    CompletionQuestion, MultipleSelectionQuestion, SingleSelectionQuestion,
};
use crate::domain::entity::{id::Id, question::Question};
use crate::domain::repository::question::{
    AnswerInvalidSnafu, InsufficientSnafu, QuestionInvalidSnafu, QuestionRepository,
    QuestionRepositoryError, SelectCount,
};

use crate::repository::connection::AsyncSqlitePool;

pub struct QuestionSqliteRepository {
    pool: Arc<AsyncSqlitePool>,
}

impl QuestionSqliteRepository {
    pub fn new(pool: Arc<AsyncSqlitePool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl QuestionRepository for QuestionSqliteRepository {
    async fn insert_single_selection(
        &self,
        content: String,
        options: Vec<String>,
        answer: u32,
    ) -> Result<(), QuestionRepositoryError> {
        use crate::repository::schema::questions::dsl;

        let mut connection = self
            .pool
            .get()
            .await
            .map_err(|err| -> Box<dyn Error + Send> { Box::new(err) })
            .whatever_context("Could not connect to database")?;

        diesel::insert_into(dsl::questions)
            .values(DbQuestionInsertion::new_single_selection(
                content, options, answer,
            )?)
            .execute(&mut connection)
            .await
            .map_err(|err| -> Box<dyn Error + Send> { Box::new(err) })
            .whatever_context("Could not save question (single-selection) to database")?;

        Ok(())
    }

    async fn insert_multiple_selection(
        &self,
        content: String,
        options: Vec<String>,
        answer: Vec<u32>,
    ) -> Result<(), QuestionRepositoryError> {
        use crate::repository::schema::questions::dsl;

        let mut connection = self
            .pool
            .get()
            .await
            .map_err(|err| -> Box<dyn Error + Send> { Box::new(err) })
            .whatever_context("Could not connect to database")?;

        DbQuestionInsertion::new_multiple_selection(content, options, answer)?
            .insert_into(dsl::questions)
            .execute(&mut connection)
            .await
            .map_err(|err| -> Box<dyn Error + Send> { Box::new(err) })
            .whatever_context("Could not save question (multiple-selection) to database")?;

        Ok(())
    }

    async fn insert_completion(
        &self,
        content: String,
        answer: String,
    ) -> Result<(), QuestionRepositoryError> {
        use crate::repository::schema::questions::dsl;

        let mut connection = self
            .pool
            .get()
            .await
            .map_err(|err| -> Box<dyn Error + Send> { Box::new(err) })
            .whatever_context("Could not connect to database")?;

        DbQuestionInsertion::new_completion(content, answer)?
            .insert_into(dsl::questions)
            .execute(&mut connection)
            .await
            .map_err(|err| -> Box<dyn Error + Send> { Box::new(err) })
            .whatever_context("Could not save question (completion) to database")?;

        Ok(())
    }

    async fn select_questions(
        &self,
        select_count: SelectCount,
    ) -> Result<Vec<Question>, QuestionRepositoryError> {
        use crate::repository::schema::questions::dsl;

        let mut connection = self
            .pool
            .get()
            .await
            .map_err(|err| -> Box<dyn Error + Send> { Box::new(err) })
            .whatever_context("Could not connect to database")?;

        let single_selection: Vec<DbQuestion> = dsl::questions
            .select(DbQuestion::as_select())
            .filter(dsl::kind.eq(DbQuestionKind::SINGLE_SELECTION))
            .limit(select_count.single_selection as i64)
            .order_by(diesel::dsl::sql::<Integer>("RANDOM()"))
            .load(&mut connection)
            .await
            .map_err(|err| -> Box<dyn Error + Send> { Box::new(err) })
            .whatever_context("Could not load questions (single-selection) from database")?;

        ensure!(
            single_selection.len() >= select_count.single_selection,
            InsufficientSnafu {
                which: "single-selection",
                expected: select_count.single_selection,
                total: single_selection.len(),
            }
        );

        let multiple_selection: Vec<DbQuestion> = dsl::questions
            .select(DbQuestion::as_select())
            .filter(dsl::kind.eq(DbQuestionKind::MULTIPLE_SELECTION))
            .limit(select_count.multiple_selection as i64)
            .order_by(diesel::dsl::sql::<Integer>("RANDOM()"))
            .load(&mut connection)
            .await
            .map_err(|err| -> Box<dyn Error + Send> { Box::new(err) })
            .whatever_context("Could not load questions (multiple-selection) from database")?;

        ensure!(
            multiple_selection.len() >= select_count.multiple_selection,
            InsufficientSnafu {
                which: "multiple-selection",
                expected: select_count.multiple_selection,
                total: multiple_selection.len(),
            }
        );

        let completion: Vec<DbQuestion> = dsl::questions
            .select(DbQuestion::as_select())
            .filter(dsl::kind.eq(DbQuestionKind::COMPLETION))
            .limit(select_count.completion as i64)
            .order_by(diesel::dsl::sql::<Integer>("RANDOM()"))
            .load(&mut connection)
            .await
            .map_err(|err| -> Box<dyn Error + Send> { Box::new(err) })
            .whatever_context("Could not load questions (completion) from database")?;

        ensure!(
            completion.len() >= select_count.completion,
            InsufficientSnafu {
                which: "completion",
                expected: select_count.completion,
                total: completion.len(),
            }
        );

        let mut res = Vec::new();

        for q in single_selection {
            res.push(q.into_single_question().into());
        }

        for q in multiple_selection {
            res.push(q.into_multiple_question().into());
        }

        for q in completion {
            res.push(q.into_completion().into());
        }

        Ok(res)
    }

    async fn select_questions_by_id(
        &self,
        id: Vec<Id>,
    ) -> Result<Vec<Question>, QuestionRepositoryError> {
        use crate::repository::schema::questions::dsl;

        let mut connection = self
            .pool
            .get()
            .await
            .map_err(|err| -> Box<dyn Error + Send> { Box::new(err) })
            .whatever_context("Could not connect to database")?;

        let mut res = Vec::with_capacity(id.len());

        for i in id {
            let question: DbQuestion = dsl::questions
                .select(DbQuestion::as_select())
                .filter(dsl::id.eq(i.inner() as i32))
                .first(&mut connection)
                .await
                .optional()
                .map_err(|err| -> Box<dyn Error + Send> { Box::new(err) })
                .whatever_context("Could not load question from database by ID")?
                .unwrap_or_else(|| unreachable!("Question of given ID should exist"));

            let question = match question.kind {
                DbQuestionKind::SINGLE_SELECTION => question.into_single_question().into(),
                DbQuestionKind::MULTIPLE_SELECTION => question.into_multiple_question().into(),
                DbQuestionKind::COMPLETION => question.into_completion().into(),
                _ => unreachable!(),
            };

            res.push(question);
        }

        Ok(res)
    }
}

impl Debug for QuestionSqliteRepository {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "QuestionSqliteRepository {{ pool: Arc<AsyncSqlitePool> }}"
        )
    }
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::repository::schema::questions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
struct DbQuestion {
    id: i32,
    kind: i32,
    content: String,
    option0: Option<String>,
    option1: Option<String>,
    option2: Option<String>,
    option3: Option<String>,
    answer: String,
}

impl DbQuestion {
    fn into_single_question(self) -> SingleSelectionQuestion {
        let answer = self
            .answer
            .parse()
            .unwrap_or_else(|_| unreachable!("Answer of single-selection should be a digit"));

        let answer = SingleSelectionAnswer::<StandardSource>::try_new(answer)
            .unwrap_or_else(|_| unreachable!("Answer should be already validated"));

        SingleSelectionQuestion::try_new(
            (self.id as usize).into(),
            self.content,
            vec![
                self.option0
                    .unwrap_or_else(|| unreachable!("Option should not be None")),
                self.option1
                    .unwrap_or_else(|| unreachable!("Option should not be None")),
                self.option2
                    .unwrap_or_else(|| unreachable!("Option should not be None")),
                self.option3
                    .unwrap_or_else(|| unreachable!("Option should not be None")),
            ],
            answer,
        )
        .unwrap_or_else(|_| unreachable!("Question should be already validated"))
    }

    fn into_multiple_question(self) -> MultipleSelectionQuestion {
        let answers = self.answer.chars().map(|c| c as u32 - '0' as u32).collect();
        let answer = MultipleSelectionAnswer::<StandardSource>::try_new(answers)
            .unwrap_or_else(|_| unreachable!("Answer should be already validated"));

        MultipleSelectionQuestion::try_new(
            (self.id as usize).into(),
            self.content,
            vec![
                self.option0
                    .unwrap_or_else(|| unreachable!("Option should not be None")),
                self.option1
                    .unwrap_or_else(|| unreachable!("Option should not be None")),
                self.option2
                    .unwrap_or_else(|| unreachable!("Option should not be None")),
                self.option3
                    .unwrap_or_else(|| unreachable!("Option should not be None")),
            ],
            answer,
        )
        .unwrap_or_else(|_| unreachable!("Question should be already validated"))
    }

    fn into_completion(self) -> CompletionQuestion {
        let answer = CompletionAnswer::<StandardSource>::try_new(self.answer)
            .unwrap_or_else(|_| unreachable!("Answer should be already validated"));

        CompletionQuestion::try_new((self.id as usize).into(), self.content, answer)
            .unwrap_or_else(|_| unreachable!("Question should be already validated"))
    }
}

struct DbQuestionKind;

impl DbQuestionKind {
    const SINGLE_SELECTION: i32 = 0;
    const MULTIPLE_SELECTION: i32 = 1;
    const COMPLETION: i32 = 2;
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::repository::schema::questions)]
struct DbQuestionInsertion {
    kind: i32,
    content: String,
    option0: Option<String>,
    option1: Option<String>,
    option2: Option<String>,
    option3: Option<String>,
    answer: String,
}

impl DbQuestionInsertion {
    #[allow(clippy::get_first)]
    fn new_single_selection(
        content: String,
        options: Vec<String>,
        answer: u32,
    ) -> Result<Self, QuestionRepositoryError> {
        {
            let checked_answer = SingleSelectionAnswer::<StandardSource>::try_new(answer)
                .context(AnswerInvalidSnafu)?;
            let _checked_question = SingleSelectionQuestion::try_new(
                0.into(),
                content.clone(),
                options.clone(),
                checked_answer,
            )
            .context(QuestionInvalidSnafu)?;
        }

        Ok(Self {
            kind: DbQuestionKind::SINGLE_SELECTION,
            content,
            option0: options.get(0).cloned(),
            option1: options.get(1).cloned(),
            option2: options.get(2).cloned(),
            option3: options.get(3).cloned(),
            answer: answer.to_string(),
        })
    }

    #[allow(clippy::get_first)]
    fn new_multiple_selection(
        content: String,
        options: Vec<String>,
        answer: Vec<u32>,
    ) -> Result<Self, QuestionRepositoryError> {
        {
            let checked_answer = MultipleSelectionAnswer::<StandardSource>::try_new(answer.clone())
                .context(AnswerInvalidSnafu)?;
            let _checked_question = MultipleSelectionQuestion::try_new(
                0.into(),
                content.clone(),
                options.clone(),
                checked_answer,
            )
            .context(QuestionInvalidSnafu)?;
        }

        Ok(Self {
            kind: DbQuestionKind::MULTIPLE_SELECTION,
            content,
            option0: options.get(0).cloned(),
            option1: options.get(1).cloned(),
            option2: options.get(2).cloned(),
            option3: options.get(3).cloned(),
            answer: answer.iter().fold("".into(), |mut res, option| {
                res.push_str(&option.to_string());
                res
            }),
        })
    }

    fn new_completion(content: String, answer: String) -> Result<Self, QuestionRepositoryError> {
        {
            let checked_answer = CompletionAnswer::<StandardSource>::try_new(answer.clone())
                .context(AnswerInvalidSnafu)?;
            let _checked_question =
                CompletionQuestion::try_new(0.into(), content.clone(), checked_answer)
                    .context(QuestionInvalidSnafu)?;
        }

        Ok(Self {
            kind: DbQuestionKind::COMPLETION,
            content,
            option0: None,
            option1: None,
            option2: None,
            option3: None,
            answer,
        })
    }
}
