use std::sync::Arc;

use snafu::{prelude::*, OptionExt, ResultExt};
use tokio::sync::oneshot::{self, Sender as OneshotSender};
use tokio::time::Duration;

use crate::domain::entity::id::Id;
use crate::domain::entity::test::{Submission, Test};
use crate::domain::entity::user::User;
use crate::domain::repository::question::QuestionRepository;
use crate::domain::repository::score::{Record, ScoreRepository, ScoreRepositoryError};
use crate::domain::session::base::{Command, Session, SessionBase};
use crate::domain::session::test::{TestSession, TestSessionError, TestSummary};

use super::test::TestSessionCommand;

#[derive(Debug)]
pub struct LoginSession {
    base: SessionBase<Self>,
    question_repository: Arc<dyn QuestionRepository>,
    score_repository: Arc<dyn ScoreRepository>,
    user: Option<User>,
}

impl LoginSession {
    pub fn new(
        base: SessionBase<Self>,
        question_repository: Arc<dyn QuestionRepository>,
        score_repository: Arc<dyn ScoreRepository>,
    ) -> Self {
        Self {
            base,
            question_repository,
            score_repository,
            user: None,
        }
    }

    fn handle_login(&mut self, user: User) -> Result<Id, LoginSessionError> {
        if let Some(previous) = &self.user {
            AlreadyLoggedInSnafu {
                previous: previous.clone(),
            }
            .fail()
        } else {
            self.user = Some(user);
            Ok(self.id())
        }
    }

    async fn handle_start(&mut self) -> Result<Test, LoginSessionError> {
        ensure!(self.user.is_some(), NotLoggedInSnafu);

        let question_repository = Arc::clone(&self.question_repository);
        let score_repository = Arc::clone(&self.score_repository);
        let id = self
            .spawn(|base| TestSession::new(base, question_repository, score_repository))
            .unwrap_or_else(|| unreachable!("A TestSession should start"));

        let commander = self
            .base
            .sub_sessions
            .get(&id)
            .context(SessionNotFoundSnafu { id })?;

        let (responder, receiver) = oneshot::channel();
        let _ = commander
            .send(Command::Extra(TestSessionCommand::Generate { responder }))
            .await;

        let test = receiver
            .await
            .unwrap_or_else(|_| unreachable!("TestSession should send response back"))
            .context(TestSnafu)?;

        Ok(test)
    }

    async fn handle_submit(
        &mut self,
        test_id: Id,
        submission: Submission,
    ) -> Result<TestSummary, LoginSessionError> {
        let user = self.user.clone().context(NotLoggedInSnafu)?;
        let (responder, receiver) = oneshot::channel();
        let commander = self
            .base
            .sub_sessions
            .get(&test_id)
            .context(SessionNotFoundSnafu { id: test_id })?;

        let _ = commander
            .send(Command::Extra(TestSessionCommand::Submit {
                user,
                test_id,
                submission,
                responder,
            }))
            .await;

        let res = receiver
            .await
            .unwrap_or_else(|_| unreachable!("TestSession should send response back"))
            .context(TestSnafu)?;

        Ok(res)
    }

    async fn handle_query(&mut self, kind: QueryKind) -> Result<Record, LoginSessionError> {
        let user = self.user.clone().context(NotLoggedInSnafu)?;

        match kind {
            QueryKind::Best => self
                .score_repository
                .query_best(&user)
                .await
                .context(QuerySnafu),
            QueryKind::Latest => self
                .score_repository
                .query_latest(&user)
                .await
                .context(QuerySnafu),
        }
    }

    async fn handle_query_all(&mut self) -> Result<Vec<Record>, LoginSessionError> {
        let user = self.user.clone().context(NotLoggedInSnafu)?;
        self.score_repository
            .query_all_sorted(&user)
            .await
            .context(QuerySnafu)
    }
}

#[async_trait::async_trait]
impl Session for LoginSession {
    const SESSION_EXPIRE_TIMEOUT: Duration = Duration::from_secs(60 * 45);
    const CANCEL_AWAIT_TIMEOUT: Duration = Duration::from_secs(5);

    type ExtraCommand = LoginSessionCommand;
    type SubSession = TestSession;

    fn base(&self) -> &SessionBase<Self> {
        &self.base
    }

    fn base_mut(&mut self) -> &mut SessionBase<Self> {
        &mut self.base
    }

    async fn handle(&mut self, command: Self::ExtraCommand) {
        match command {
            Self::ExtraCommand::Login { user, responder } => {
                let res = self.handle_login(user);
                let _ = responder.send(res);
            }
            Self::ExtraCommand::Start { responder } => {
                let res = self.handle_start().await;
                let _ = responder.send(res);
            }
            Self::ExtraCommand::Submit {
                test_id,
                submission,
                responder,
            } => {
                let res = self.handle_submit(test_id, submission).await;
                let _ = responder.send(res);
            }
            Self::ExtraCommand::Query { kind, responder } => {
                let res = self.handle_query(kind).await;
                let _ = responder.send(res);
            }
            Self::ExtraCommand::QueryAll { responder } => {
                let res = self.handle_query_all().await;
                let _ = responder.send(res);
            }
        }
    }

    async fn finalize(&mut self) {}
}

#[derive(Debug)]
pub enum LoginSessionCommand {
    Login {
        user: User,
        responder: OneshotSender<Result<Id, LoginSessionError>>,
    },
    Start {
        responder: OneshotSender<Result<Test, LoginSessionError>>,
    },
    Submit {
        test_id: Id,
        submission: Submission,
        responder: OneshotSender<Result<TestSummary, LoginSessionError>>,
    },
    Query {
        kind: QueryKind,
        responder: OneshotSender<Result<Record, LoginSessionError>>,
    },
    QueryAll {
        responder: OneshotSender<Result<Vec<Record>, LoginSessionError>>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryKind {
    Best,
    Latest,
}

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum LoginSessionError {
    #[snafu(display("Session is not logged in"))]
    NotLoggedIn,
    #[snafu(display("Already logged in as {previous}"))]
    AlreadyLoggedIn { previous: User },
    #[snafu(display("Could not find session {id}"))]
    SessionNotFound { id: Id },
    #[snafu(display("Could not handle test"))]
    Test { source: TestSessionError },
    #[snafu(display("Could not query user's score"))]
    Query { source: ScoreRepositoryError },
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use tokio::sync::mpsc::{self, Receiver as MpscReceiver, Sender as MpscSender};
    use tokio::time::Instant;

    use crate::domain::entity::id::SequentialIdAllocator;
    use crate::domain::entity::question::Question;
    use crate::domain::entity::score::Score;
    use crate::domain::repository::question::{MockQuestionRepository, SelectCount};
    use crate::domain::repository::score::MockScoreRepository;
    use crate::domain::session::base::Report;

    use super::*;

    #[tokio::test(start_paused = true)]
    async fn login_session_start_submit() {
        let (question_repository, score_repository, _) = new_repository();
        let (mut session, _, _) = new_login_session(question_repository, score_repository);

        let user = User::try_new("user").unwrap();
        session.handle_login(user).unwrap();

        let id = 1.into();
        let res = session.handle_start().await.unwrap();
        assert_eq!(res, Test::new(id, Vec::new()));

        let res = session.handle_submit(id, new_submission()).await.unwrap();
        assert_eq!(
            res,
            TestSummary {
                score: Score::try_new(100f32).unwrap(),
                duration: Duration::from_secs(0)
            }
        );
    }

    #[tokio::test(start_paused = true)]
    async fn login_session_query() {
        let (question_repository, score_repository, _) = new_repository();
        let (mut session, _, _) = new_login_session(question_repository, score_repository);

        let user = User::try_new("user").unwrap();
        session.handle_login(user).unwrap();

        assert_eq!(
            session.handle_query(QueryKind::Best).await.unwrap(),
            Record {
                score: Score::try_new(100f32).unwrap(),
                end_time: Instant::now(),
                duration: Duration::from_secs(0),
            }
        );

        assert_eq!(
            session.handle_query(QueryKind::Latest).await.unwrap(),
            Record {
                score: Score::try_new(100f32).unwrap(),
                end_time: Instant::now(),
                duration: Duration::from_secs(0),
            }
        );

        assert_eq!(
            session.handle_query_all().await.unwrap(),
            vec![Record {
                score: Score::try_new(100f32).unwrap(),
                end_time: Instant::now(),
                duration: Duration::from_secs(0),
            }]
        );
    }

    #[tokio::test(start_paused = true)]
    async fn login_session_already_logged_in() {
        let (question_repository, score_repository, _) = new_repository();
        let (mut session, _, _) = new_login_session(question_repository, score_repository);

        let user = User::try_new("user").unwrap();
        session.handle_login(user).unwrap();

        let user2 = User::try_new("user2").unwrap();
        assert!(matches!(
            session.handle_login(user2),
            Err(LoginSessionError::AlreadyLoggedIn { .. }),
        ));
    }

    #[tokio::test(start_paused = true)]
    async fn login_session_not_logged_in() {
        let (question_repository, score_repository, _) = new_repository();

        {
            let (mut session, _, _) = new_login_session(
                Arc::clone(&question_repository),
                Arc::clone(&score_repository),
            );

            assert!(matches!(
                session.handle_start().await,
                Err(LoginSessionError::NotLoggedIn),
            ));
        }

        {
            let (mut session, _, _) = new_login_session(
                Arc::clone(&question_repository),
                Arc::clone(&score_repository),
            );

            assert!(matches!(
                session.handle_submit(1.into(), new_submission()).await,
                Err(LoginSessionError::NotLoggedIn),
            ));
        }

        {
            let (mut session, _, _) = new_login_session(
                Arc::clone(&question_repository),
                Arc::clone(&score_repository),
            );

            assert!(matches!(
                session.handle_query(QueryKind::Best).await,
                Err(LoginSessionError::NotLoggedIn),
            ));
        }

        {
            let (mut session, _, _) = new_login_session(
                Arc::clone(&question_repository),
                Arc::clone(&score_repository),
            );

            assert!(matches!(
                session.handle_query_all().await,
                Err(LoginSessionError::NotLoggedIn),
            ));
        }
    }

    #[tokio::test(start_paused = true)]
    async fn login_session_session_not_found() {
        let (question_repository, score_repository, _) = new_repository();
        let (mut session, _, _) = new_login_session(question_repository, score_repository);

        let user = User::try_new("user").unwrap();
        session.handle_login(user).unwrap();

        session.handle_start().await.unwrap();

        assert!(matches!(
            session.handle_submit(2.into(), new_submission()).await,
            Err(LoginSessionError::SessionNotFound { .. })
        ));
    }

    fn new_login_session(
        question_repository: Arc<dyn QuestionRepository>,
        score_repository: Arc<dyn ScoreRepository>,
    ) -> (
        LoginSession,
        MpscSender<Command<LoginSession>>,
        MpscReceiver<Report>,
    ) {
        let id_allocator = Arc::new(SequentialIdAllocator::new());
        let (commander, command) = mpsc::channel(4);
        let (reporter, report) = mpsc::channel(4);
        let base = SessionBase::new(id_allocator, command, reporter);
        let session = LoginSession::new(base, question_repository, score_repository);
        (session, commander, report)
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
        score_repository.expect_query_latest().returning(|_| {
            Ok(Record {
                score: Score::try_new(100f32).unwrap(),
                end_time: Instant::now(),
                duration: Duration::from_secs(0),
            })
        });
        score_repository.expect_query_best().returning(|_| {
            Ok(Record {
                score: Score::try_new(100f32).unwrap(),
                end_time: Instant::now(),
                duration: Duration::from_secs(0),
            })
        });
        score_repository.expect_query_all_sorted().returning(|_| {
            Ok(vec![Record {
                score: Score::try_new(100f32).unwrap(),
                end_time: Instant::now(),
                duration: Duration::from_secs(0),
            }])
        });

        (
            Arc::new(question_repository),
            Arc::new(score_repository),
            SelectCount {
                single_selection: 0,
                multiple_selection: 0,
                completion: 0,
            },
        )
    }

    fn new_questions() -> Vec<Question> {
        Vec::new()
    }

    fn new_submission() -> Submission {
        Submission::new(HashMap::new())
    }
}
