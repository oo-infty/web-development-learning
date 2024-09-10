use std::collections::HashMap;
use std::sync::Arc;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use snafu::ResultExt;

use crate::domain::application::{Core, CoreError};
use crate::domain::entity::answer::{
    CompletionAnswer, MultipleSelectionAnswer, SingleSelectionAnswer, SubmissionSource,
};
use crate::domain::entity::test::Submission;
use crate::domain::session::login::LoginSessionError;
use crate::inbound::error::{
    ApiError, DataInvalidSnafu, NotLoggedInSnafu, TestInvalidOrExpiredSnafu, UnknownSnafu,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitRequest {
    login_id: usize,
    test_id: usize,
    answers: Vec<AnswerWithId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnswerWithId {
    id: usize,
    #[serde(flatten)]
    answer: AnswerVariant,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AnswerVariant {
    SingleSelection { answer: u32 },
    MultipleSelection { answer: Vec<u32> },
    Completion { answer: String },
}

#[axum_macros::debug_handler]
pub async fn handle_submit(
    State(core): State<Arc<Core>>,
    Json(request): Json<SubmitRequest>,
) -> Result<(), ApiError> {
    let mut answers = HashMap::new();

    for AnswerWithId { id, answer } in request.answers {
        let answer = match answer {
            AnswerVariant::SingleSelection { answer } => {
                SingleSelectionAnswer::<SubmissionSource>::try_new(answer)
                    .map(Into::into)
                    .map_err(Into::into)
                    .context(DataInvalidSnafu)?
            }
            AnswerVariant::MultipleSelection { answer } => {
                MultipleSelectionAnswer::<SubmissionSource>::try_new(answer)
                    .map(Into::into)
                    .map_err(Into::into)
                    .context(DataInvalidSnafu)?
            }
            AnswerVariant::Completion { answer } => {
                CompletionAnswer::<SubmissionSource>::try_new(answer)
                    .map(Into::into)
                    .map_err(Into::into)
                    .context(DataInvalidSnafu)?
            }
        };

        let _ = answers.insert(id.into(), answer);
    }

    let submission = Submission::new(answers);

    let res = core
        .submit(request.login_id.into(), request.test_id.into(), submission)
        .await;

    if let Err(err) = res {
        match err {
            CoreError::LoginSession {
                source: LoginSessionError::SessionNotFound { .. },
                ..
            } => TestInvalidOrExpiredSnafu.fail(),
            CoreError::SessionNotFound { .. } => NotLoggedInSnafu.fail(),
            _ => Err(err.into()).context(UnknownSnafu),
        }
    } else {
        Ok(())
    }
}

#[test]
fn ttt() {
    let s = SubmitRequest {
        login_id: 1,
        test_id: 1,
        answers: vec![
            AnswerWithId {
                id: 0,
                answer: AnswerVariant::MultipleSelection { answer: vec![0] },
            },
            AnswerWithId {
                id: 1,
                answer: AnswerVariant::Completion {
                    answer: "aa".into(),
                },
            },
        ],
    };
    println!("{}", serde_json::to_string_pretty(&s).unwrap());
}
