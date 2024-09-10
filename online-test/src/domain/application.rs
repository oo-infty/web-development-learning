use std::collections::HashMap;
use std::sync::Arc;

use snafu::prelude::*;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::sync::oneshot;
use tokio::sync::{Mutex, RwLock};

use crate::domain::entity::id::{Id, SequentialIdAllocator};
use crate::domain::entity::test::{Submission, Test};
use crate::domain::entity::user::User;
use crate::domain::repository::question::QuestionRepository;
use crate::domain::repository::score::{Record, ScoreRepository};
use crate::domain::session::base::{Command, Report, Session, SessionBase};
use crate::domain::session::login::{
    LoginSession, LoginSessionCommand, LoginSessionError, QueryKind,
};
use crate::domain::session::test::TestSummary;

#[derive(Debug)]
pub struct Core {
    question_repository: Arc<dyn QuestionRepository>,
    score_repository: Arc<dyn ScoreRepository>,
    id_allocator: Arc<SequentialIdAllocator>,
    sessions: RwLock<HashMap<Id, Sender<Command<LoginSession>>>>,
    report: Mutex<Receiver<Report>>,
    sub_reporter: Sender<Report>,
}

impl Core {
    pub fn new(
        question_repository: Arc<dyn QuestionRepository>,
        score_repository: Arc<dyn ScoreRepository>,
    ) -> Self {
        let (sub_reporter, report) = mpsc::channel(128);

        Self {
            question_repository,
            score_repository,
            sessions: RwLock::new(HashMap::new()),
            id_allocator: Arc::new(SequentialIdAllocator::new()),
            report: Mutex::new(report),
            sub_reporter,
        }
    }

    pub async fn login(&self, user: User) -> Result<Id, CoreError> {
        self.handle_exited_sessions().await;

        let (commander, command) = mpsc::channel(4);
        let base = SessionBase::new(
            Arc::clone(&self.id_allocator),
            command,
            self.sub_reporter.clone(),
        )
        .await;

        let mut session = LoginSession::new(
            base,
            Arc::clone(&self.question_repository),
            Arc::clone(&self.score_repository),
        );
        let id = session.id();
        self.sessions.write().await.insert(id, commander.clone());
        tokio::spawn(async move { session.run().await });

        let (responder, respond) = oneshot::channel();

        let _ = commander
            .send(Command::Extra(LoginSessionCommand::Login {
                user,
                responder,
            }))
            .await;

        let login_id = respond
            .await
            .unwrap_or_else(|_| unreachable!("LoginSession should start"))
            .context(LoginSessionSnafu { id })?;

        Ok(login_id)
    }

    pub async fn start(&self, login_id: Id) -> Result<Test, CoreError> {
        self.handle_exited_sessions().await;

        let (responder, respond) = oneshot::channel();

        let _ = self
            .sessions
            .read()
            .await
            .get(&login_id)
            .context(SessionNotFoundSnafu { id: login_id })?
            .send(Command::Extra(LoginSessionCommand::Start { responder }))
            .await;

        let res = respond
            .await
            .unwrap_or_else(|_| unreachable!("LoginSession should send response back"))
            .context(LoginSessionSnafu { id: login_id })?;

        Ok(res)
    }

    pub async fn submit(
        &self,
        login_id: Id,
        test_id: Id,
        submission: Submission,
    ) -> Result<TestSummary, CoreError> {
        self.handle_exited_sessions().await;

        let (responder, respond) = oneshot::channel();

        let _ = self
            .sessions
            .read()
            .await
            .get(&login_id)
            .context(SessionNotFoundSnafu { id: login_id })?
            .send(Command::Extra(LoginSessionCommand::Submit {
                test_id,
                submission,
                responder,
            }))
            .await;

        let res = respond
            .await
            .unwrap_or_else(|_| unreachable!("LoginSession should send response back"))
            .context(LoginSessionSnafu { id: login_id })?;

        Ok(res)
    }

    pub async fn query(&self, login_id: Id, kind: QueryKind) -> Result<Record, CoreError> {
        self.handle_exited_sessions().await;

        let (responder, respond) = oneshot::channel();

        let _ = self
            .sessions
            .read()
            .await
            .get(&login_id)
            .context(SessionNotFoundSnafu { id: login_id })?
            .send(Command::Extra(LoginSessionCommand::Query {
                kind,
                responder,
            }))
            .await;

        let res = respond
            .await
            .unwrap_or_else(|_| unreachable!("LoginSession should send response back"))
            .context(LoginSessionSnafu { id: login_id })?;

        Ok(res)
    }

    pub async fn query_all(&self, login_id: Id) -> Result<Vec<Record>, CoreError> {
        self.handle_exited_sessions().await;

        let (responder, respond) = oneshot::channel();

        let _ = self
            .sessions
            .read()
            .await
            .get(&login_id)
            .context(SessionNotFoundSnafu { id: login_id })?
            .send(Command::Extra(LoginSessionCommand::QueryAll { responder }))
            .await;

        let res = respond
            .await
            .unwrap_or_else(|_| unreachable!("LoginSession should send response back"))
            .context(LoginSessionSnafu { id: login_id })?;

        Ok(res)
    }

    async fn handle_exited_sessions(&self) {
        loop {
            let report = self.report.lock().await.try_recv();

            if let Ok(Report::Exited { id }) = report {
                let _ = self.sessions.write().await.remove(&id);
            } else if let Err(TryRecvError::Empty) = report {
                break;
            }
        }
    }
}

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum CoreError {
    #[snafu(display("Error occurred in login session {id}"))]
    LoginSession { id: Id, source: LoginSessionError },
    #[snafu(display("Could not find session {id}"))]
    SessionNotFound { id: Id },
}
