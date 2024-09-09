use std::collections::HashMap;
use std::convert::Infallible;
use std::fmt::Debug;
use std::sync::Arc;

use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::time::{interval, timeout, Duration, Interval};

use crate::domain::entity::id::{Id, SequentialIdAllocator};

#[async_trait::async_trait]
pub trait Session: Debug + Sized + Send + 'static {
    const SESSION_EXPIRE_TIMEOUT: Duration;
    const CANCEL_AWAIT_TIMEOUT: Duration;

    type ExtraCommand: Debug + Send;
    type SubSession: Session;

    fn base(&self) -> &SessionBase<Self>;

    fn base_mut(&mut self) -> &mut SessionBase<Self>;

    async fn handle(&mut self, command: Self::ExtraCommand);

    async fn finalize(&mut self);

    fn id(&self) -> Id {
        self.base().id
    }

    async fn run(&mut self) {
        loop {
            let mut command = None;
            let mut report = None;
            let mut expire = None;

            {
                let base = self.base_mut();
                tokio::select! {
                    recv = base.command.recv() => {
                        command = Some(recv.unwrap_or(Command::Cancel));
                    }
                    recv = base.sub_report.recv() => {
                        report = Some(recv);
                    }
                    recv = base.expire_timer.tick() => {
                        expire = Some(recv);
                    }
                }
            }

            match command {
                Some(Command::Extra(command)) => {
                    self.handle(command).await;

                    if self.base().exit_token {
                        break;
                    }
                }
                Some(Command::Cancel) => {
                    self.cancel_sub_sessions().await;
                    break;
                }
                None => {}
            }

            match report {
                Some(Some(Report::Exited { id })) => {
                    self.base_mut().sub_sessions.remove(&id);
                }
                Some(None) => break,
                None => {}
            }

            if expire.is_some() {
                self.cancel_sub_sessions().await;
                break;
            }
        }

        self.finalize().await;

        let id = self.id();
        let _ = self.base_mut().reporter.send(Report::Exited { id }).await;
    }

    async fn cancel_sub_sessions(&mut self) {
        drop(self.base_mut().sub_reporter.take());
        let mut running = 0usize;

        for sub in self.base_mut().sub_sessions.values_mut() {
            let res = sub.send(Command::Cancel).await;
            running += res.map_or(0, |_| 1);
        }

        let _ = timeout(Self::CANCEL_AWAIT_TIMEOUT, async {
            while running > 0 {
                let res = self.base_mut().sub_report.recv().await;
                running -= res.map_or(0, |_| 1);
            }
        })
        .await;
    }

    fn spawn<F>(&mut self, construtor: F) -> Option<Id>
    where
        F: FnOnce(SessionBase<Self::SubSession>) -> Self::SubSession + Send + 'static,
    {
        let id_allocator = Arc::clone(&self.base().id_allocator);
        let (commander, command) = channel(4);
        let reporter = self.base().sub_reporter.clone()?;
        let mut sub_base = SessionBase::new(id_allocator, command, reporter);

        let id = sub_base.id;
        self.base_mut().sub_sessions.insert(id, commander);

        tokio::spawn(async move {
            sub_base.expire_timer.tick().await;
            let mut sub = construtor(sub_base);
            sub.run().await;
        });

        Some(id)
    }

    fn request_exit(&mut self) {
        self.base_mut().exit_token = true;
    }

    fn reset_expire(&mut self) {
        self.base_mut().expire_timer.reset();
    }
}

#[derive(Debug)]
pub enum NoneSession {}

#[async_trait::async_trait]
impl Session for NoneSession {
    const SESSION_EXPIRE_TIMEOUT: Duration = Duration::from_secs(0);
    const CANCEL_AWAIT_TIMEOUT: Duration = Duration::from_secs(0);

    type ExtraCommand = Infallible;
    type SubSession = NoneSession;

    fn base(&self) -> &SessionBase<Self> {
        unreachable!()
    }

    fn base_mut(&mut self) -> &mut SessionBase<Self> {
        unreachable!()
    }

    async fn handle(&mut self, _command: Self::ExtraCommand) {
        unreachable!()
    }

    async fn finalize(&mut self) {
        unreachable!()
    }
}

#[derive(Debug)]
pub enum Command<S: Session> {
    Cancel,
    Extra(S::ExtraCommand),
}

#[derive(Debug)]
pub enum Report {
    Exited { id: Id },
}

#[derive(Debug)]
pub struct SessionBase<S: Session> {
    pub(super) id: Id,
    pub(super) id_allocator: Arc<SequentialIdAllocator>,
    pub(super) command: Receiver<Command<S>>,
    pub(super) sub_sessions: HashMap<Id, Sender<Command<S::SubSession>>>,
    pub(super) reporter: Sender<Report>,
    pub(super) sub_report: Receiver<Report>,
    pub(super) sub_reporter: Option<Sender<Report>>,
    pub(super) expire_timer: Interval,
    pub(super) exit_token: bool,
}

impl<S: Session> SessionBase<S> {
    pub fn new(
        id_allocator: Arc<SequentialIdAllocator>,
        command: Receiver<Command<S>>,
        reporter: Sender<Report>,
    ) -> Self {
        let id = id_allocator.allocate();
        let (sub_reporter, sub_report) = channel(4);

        SessionBase {
            id,
            id_allocator,
            command,
            sub_sessions: HashMap::new(),
            reporter,
            sub_report,
            sub_reporter: Some(sub_reporter),
            expire_timer: interval(S::SESSION_EXPIRE_TIMEOUT),
            exit_token: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::sync::Mutex;

    use super::*;

    #[tokio::test(start_paused = true)]
    async fn session_handle() {
        let (commander, _, set) = SimpleSession::root().await;
        commander
            .send(Command::Extra(SimpleSessionCommand::Send))
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_millis(10)).await;
        assert_eq!(*set.lock().unwrap(), vec![0].into_iter().collect());

        set.lock().unwrap().clear();
        commander
            .send(Command::Extra(SimpleSessionCommand::Spawn))
            .await
            .unwrap();
        commander
            .send(Command::Extra(SimpleSessionCommand::Spawn))
            .await
            .unwrap();
        commander
            .send(Command::Extra(SimpleSessionCommand::Send))
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_millis(10)).await;
        assert_eq!(*set.lock().unwrap(), vec![0, 1, 2].into_iter().collect());
    }

    #[tokio::test(start_paused = true)]
    async fn session_cancel() {
        let (commander, mut report, set) = SimpleSession::root().await;
        commander
            .send(Command::Extra(SimpleSessionCommand::Spawn))
            .await
            .unwrap();
        commander
            .send(Command::Extra(SimpleSessionCommand::Spawn))
            .await
            .unwrap();
        commander
            .send(Command::Extra(SimpleSessionCommand::Send))
            .await
            .unwrap();

        tokio::time::sleep(Duration::from_millis(10)).await;
        assert_eq!(*set.lock().unwrap(), vec![0, 1, 2].into_iter().collect());
        set.lock().unwrap().clear();

        let _ = commander.send(Command::Cancel).await;
        commander
            .send(Command::Extra(SimpleSessionCommand::Send))
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_millis(10)).await;
        assert!(set.lock().unwrap().is_empty());
        assert!(matches!(
            report.recv().await.unwrap(),
            Report::Exited { .. }
        ));
    }

    #[tokio::test(start_paused = true)]
    async fn session_expired() {
        let (_commander, mut report, _set) = SimpleSession::root().await;
        tokio::time::sleep(Duration::from_millis(110)).await;
        assert!(matches!(
            report.recv().await.unwrap(),
            Report::Exited { .. }
        ));
    }

    #[derive(Debug)]
    struct SimpleSession {
        base: SessionBase<Self>,
        set: Arc<Mutex<BTreeSet<usize>>>,
    }

    impl SimpleSession {
        async fn root() -> (
            Sender<Command<Self>>,
            Receiver<Report>,
            Arc<Mutex<BTreeSet<usize>>>,
        ) {
            let id_allocator = Arc::new(SequentialIdAllocator::new());
            let (commander, command) = channel(4);
            let (reporter, report) = channel(4);
            let mut base = SessionBase::new(id_allocator, command, reporter);

            let set = Arc::new(Mutex::new(BTreeSet::new()));
            let set2 = Arc::clone(&set);

            tokio::spawn(async move {
                base.expire_timer.tick().await;
                let mut session = SimpleSession { base, set: set2 };
                session.run().await;
            });

            (commander, report, set)
        }
    }

    #[async_trait::async_trait]
    impl Session for SimpleSession {
        const SESSION_EXPIRE_TIMEOUT: Duration = Duration::from_millis(100);
        const CANCEL_AWAIT_TIMEOUT: Duration = Duration::from_millis(100);

        type ExtraCommand = SimpleSessionCommand;
        type SubSession = Self;

        fn base(&self) -> &SessionBase<Self> {
            &self.base
        }

        fn base_mut(&mut self) -> &mut SessionBase<Self> {
            &mut self.base
        }

        async fn handle(&mut self, command: Self::ExtraCommand) {
            match command {
                Self::ExtraCommand::Send => {
                    self.set.lock().unwrap().insert(self.id().inner());
                    for (_, sub) in &self.base.sub_sessions {
                        let _ = sub.send(Command::Extra(Self::ExtraCommand::Send)).await;
                    }
                }
                Self::ExtraCommand::Spawn => {
                    let set = Arc::clone(&self.set);
                    self.spawn(|base| SimpleSession { base, set }).unwrap();
                }
            };
        }

        async fn finalize(&mut self) {}
    }

    #[derive(Debug)]
    enum SimpleSessionCommand {
        Send,
        Spawn,
    }
}
