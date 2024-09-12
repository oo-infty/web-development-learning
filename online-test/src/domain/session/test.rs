use std::sync::Arc;

use chrono::{DateTime, Utc};
use snafu::prelude::*;
use tokio::sync::oneshot::Sender as OneshotSender;
use tokio::time::Duration;

use crate::domain::entity::id::Id;
use crate::domain::entity::question::AbstractQuestion;
use crate::domain::entity::score::Score;
use crate::domain::entity::test::{Submission, Test};
use crate::domain::entity::user::User;
use crate::domain::repository::question::{
    QuestionRepository, QuestionRepositoryError, SelectCount,
};
use crate::domain::repository::score::{ScoreRepository, ScoreRepositoryError};
use crate::domain::session::base::{NoneSession, Session, SessionBase};

#[derive(Debug)]
pub struct TestSession {
    base: SessionBase<Self>,
    question_repository: Arc<dyn QuestionRepository>,
    score_repository: Arc<dyn ScoreRepository>,
    question_ids: Option<Vec<Id>>,
    start_time: Option<DateTime<Utc>>,
}

impl TestSession {
    pub fn new(
        base: SessionBase<Self>,
        question_repository: Arc<dyn QuestionRepository>,
        score_repository: Arc<dyn ScoreRepository>,
    ) -> Self {
        Self {
            base,
            question_repository,
            score_repository,
            question_ids: None,
            start_time: None,
        }
    }

    async fn handle_generate(&mut self) -> Result<Test, TestSessionError> {
        const SELECT_COUNT: SelectCount = SelectCount {
            single_selection: 8,
            multiple_selection: 8,
            completion: 12,
        };

        let questions = self
            .question_repository
            .select_questions(SELECT_COUNT.clone())
            .await
            .context(GenerateSnafu)?;

        let question_ids = questions.iter().map(|q| q.id()).collect();
        self.question_ids = Some(question_ids);
        self.start_time = Some(Utc::now());
        Ok(Test::new(self.id(), questions))
    }

    async fn handle_submit(
        &mut self,
        user: User,
        test_id: Id,
        submission: Submission,
    ) -> Result<TestSummary, TestSessionError> {
        ensure!(
            test_id == self.id(),
            TestMismatchedSnafu {
                expected: self.id(),
                actual: test_id
            }
        );

        let Some(question_ids) = self.question_ids.take() else {
            return NotStartedSnafu.fail();
        };

        let questions = self
            .question_repository
            .select_questions_by_id(question_ids)
            .await
            .context(LoadQuestionsSnafu)?;

        let test = Test::new(test_id, questions);
        let score = test.grade(&submission);

        let end_time = Utc::now();
        let duration = self
            .start_time
            .take()
            .map_or(Self::SESSION_EXPIRE_TIMEOUT, |start_time| {
                (end_time - start_time).to_std().unwrap_or_default()
            });

        self.score_repository
            .insert(user, score, end_time, duration)
            .await
            .context(SaveScoreSnafu)?;

        Ok(TestSummary { score, duration })
    }
}

#[async_trait::async_trait]
impl Session for TestSession {
    const SESSION_EXPIRE_TIMEOUT: Duration = Duration::from_secs(60 * 30 + 30);
    const CANCEL_AWAIT_TIMEOUT: Duration = Duration::from_secs(5);

    type ExtraCommand = TestSessionCommand;
    type SubSession = NoneSession;

    fn base(&self) -> &SessionBase<Self> {
        &self.base
    }

    fn base_mut(&mut self) -> &mut SessionBase<Self> {
        &mut self.base
    }

    async fn handle(&mut self, command: Self::ExtraCommand) {
        match command {
            Self::ExtraCommand::Generate { responder } => {
                let res = self.handle_generate().await;
                let _ = responder.send(res);
            }
            Self::ExtraCommand::Submit {
                user,
                test_id,
                submission,
                responder,
            } => {
                let res = self.handle_submit(user, test_id, submission).await;
                let _ = responder.send(res);
                self.request_exit();
            }
        }
    }

    async fn finalize(&mut self) {}
}

#[derive(Debug)]
pub enum TestSessionCommand {
    Generate {
        responder: OneshotSender<Result<Test, TestSessionError>>,
    },
    Submit {
        user: User,
        test_id: Id,
        submission: Submission,
        responder: OneshotSender<Result<TestSummary, TestSessionError>>,
    },
}

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum TestSessionError {
    #[snafu(display("Could not generate test"))]
    Generate { source: QuestionRepositoryError },
    #[snafu(display("Could not load questions"))]
    LoadQuestions { source: QuestionRepositoryError },
    #[snafu(display("Could not save user's score"))]
    SaveScore { source: ScoreRepositoryError },
    #[snafu(display("Could not submit answer before test starts"))]
    NotStarted,
    #[snafu(display("Could not handle mismatched test {actual} in session {expected}"))]
    TestMismatched { expected: Id, actual: Id },
}

#[derive(Debug, Clone, PartialEq)]
pub struct TestSummary {
    pub score: Score,
    pub duration: Duration,
}

#[cfg(test)]
mod tests {
    use tokio::sync::mpsc::{self, Receiver as MpscReceiver, Sender as MpscSender};

    use crate::domain::entity::answer::{
        CompletionAnswer, MultipleSelectionAnswer, SingleSelectionAnswer, StandardSource,
        SubmissionSource,
    };
    use crate::domain::entity::id::SequentialIdAllocator;
    use crate::domain::entity::question::{
        CompletionQuestion, MultipleSelectionQuestion, Question, SingleSelectionQuestion,
    };
    use crate::domain::repository::question::MockQuestionRepository;
    use crate::domain::repository::score::MockScoreRepository;
    use crate::domain::session::base::{Command, Report};

    use super::*;

    #[tokio::test(start_paused = true)]
    async fn handle_generate_submit() {
        let (question_repository, score_repository, _select_count) = new_repository();
        let (mut session, _commander, _report) =
            new_test_session(question_repository, score_repository).await;
        let id = session.id();

        let actual = session.handle_generate().await.unwrap();
        let expected = Test::new(id, new_questions());
        assert_eq!(actual, expected);

        let actual = session
            .handle_submit(User::try_new("user").unwrap(), id, new_submission())
            .await
            .unwrap();
        assert_eq!(actual.score, Score::try_new(75f32).unwrap());
    }

    #[tokio::test(start_paused = true)]
    async fn handle_not_started() {
        let (question_repository, score_repository, _select_count) = new_repository();
        let (mut session, _commander, _report) =
            new_test_session(question_repository, score_repository).await;
        let id = session.id();

        assert!(matches!(
            session
                .handle_submit(User::try_new("user").unwrap(), id, new_submission())
                .await,
            Err(TestSessionError::NotStarted),
        ));
    }

    #[tokio::test(start_paused = true)]
    async fn handle_test_mismatched() {
        let (question_repository, score_repository, _select_count) = new_repository();
        let (mut session, _commander, _report) =
            new_test_session(question_repository, score_repository).await;
        let id = session.id();

        let actual = session.handle_generate().await.unwrap();
        let expected = Test::new(id, new_questions());
        assert_eq!(actual, expected);

        let err = session
            .handle_submit(User::try_new("user").unwrap(), 1.into(), new_submission())
            .await
            .unwrap_err();

        match err {
            TestSessionError::TestMismatched { expected, actual } => {
                assert_eq!(expected.inner(), 0);
                assert_eq!(actual.inner(), 1);
            }
            _ => unreachable!(),
        }
    }

    fn new_repository() -> (
        Arc<dyn QuestionRepository>,
        Arc<dyn ScoreRepository>,
        SelectCount,
    ) {
        let mut question_repository = MockQuestionRepository::new();
        question_repository
            .expect_select_questions()
            .returning(|_| Ok(new_questions()));
        question_repository
            .expect_select_questions_by_id()
            .returning(|_| Ok(new_questions()));
        let mut score_repository = MockScoreRepository::new();
        score_repository
            .expect_insert()
            .returning(|_, _, _, _| Ok(()));

        (
            Arc::new(question_repository),
            Arc::new(score_repository),
            SelectCount {
                single_selection: 2,
                multiple_selection: 1,
                completion: 1,
            },
        )
    }

    async fn new_test_session(
        question_repository: Arc<dyn QuestionRepository>,
        score_repository: Arc<dyn ScoreRepository>,
    ) -> (
        TestSession,
        MpscSender<Command<TestSession>>,
        MpscReceiver<Report>,
    ) {
        let id_allocator = Arc::new(SequentialIdAllocator::new());
        let (commander, command) = mpsc::channel(4);
        let (reporter, report) = mpsc::channel(4);
        let base = SessionBase::new(id_allocator, command, reporter).await;
        let session = TestSession::new(base, question_repository, score_repository);
        (session, commander, report)
    }

    fn new_questions() -> Vec<Question> {
        vec![
            SingleSelectionQuestion::try_new(
                0.into(),
                "0. Single-selection".into(),
                vec![
                    "option a".into(),
                    "option b".into(),
                    "option c".into(),
                    "option d".into(),
                ],
                SingleSelectionAnswer::<StandardSource>::try_new(0).unwrap(),
            )
            .unwrap()
            .into(),
            SingleSelectionQuestion::try_new(
                1.into(),
                "1. Single-selection".into(),
                vec![
                    "option a".into(),
                    "option b".into(),
                    "option c".into(),
                    "option d".into(),
                ],
                SingleSelectionAnswer::<StandardSource>::try_new(1).unwrap(),
            )
            .unwrap()
            .into(),
            MultipleSelectionQuestion::try_new(
                2.into(),
                "2. Multiple-selection".into(),
                vec![
                    "option a".into(),
                    "option b".into(),
                    "option c".into(),
                    "option d".into(),
                ],
                MultipleSelectionAnswer::<StandardSource>::try_new(vec![2, 3]).unwrap(),
            )
            .unwrap()
            .into(),
            CompletionQuestion::try_new(
                3.into(),
                "3. Completion".into(),
                CompletionAnswer::<StandardSource>::try_new("answer").unwrap(),
            )
            .unwrap()
            .into(),
        ]
    }

    fn new_submission() -> Submission {
        Submission::new(
            vec![
                SingleSelectionAnswer::<SubmissionSource>::try_new(0)
                    .unwrap()
                    .into(),
                SingleSelectionAnswer::<SubmissionSource>::try_new(0)
                    .unwrap()
                    .into(),
                MultipleSelectionAnswer::<SubmissionSource>::try_new(vec![2, 3])
                    .unwrap()
                    .into(),
                CompletionAnswer::<SubmissionSource>::try_new("answer")
                    .unwrap()
                    .into(),
            ]
            .into_iter()
            .enumerate()
            .map(|(i, a)| (i.into(), a))
            .collect(),
        )
    }
}
