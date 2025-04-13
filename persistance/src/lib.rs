pub mod questioner;

use std::{path::Path, sync::Arc};

use application::{
    env::{core_config, load_env_file},
    UnitOfWork,
};
use async_trait::async_trait;
use domain::DomainEvents;
use questioner::SqlxQuestionerRepository;
use sqlx::{
    migrate::Migrator,
    query::Query,
    sqlite::{SqliteArguments, SqlitePoolOptions},
    Pool, Sqlite,
};
use tokio::sync::Mutex;

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../migrations");

type Result<T> = std::result::Result<T, Error>;
#[derive(Debug)]
pub enum Error {
    Sqlx(sqlx::Error),
}
impl Error {
    fn get_message(self) -> String {
        match self {
            Error::Sqlx(error) => format!("{:?}", error),
        }
    }
}
impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Error::Sqlx(value)
    }
}

impl From<Error> for application::Error {
    fn from(value: Error) -> Self {
        application::Error::Persistence(value.get_message())
    }
}

// #[derive(Clone, Copy, PartialEq, Eq, Type)]
// #[sqlx(transparent)]
// pub struct UnixTimeUs(i64);

// impl UnixTimeUs {
//     pub fn now() -> Self {
//         let now = SystemTime::now();
//         let since_the_epoch = now
//             .duration_since(UNIX_EPOCH)
//             .unwrap_or(Duration::new(0, 0));

//         let unix_time = since_the_epoch.as_micros().min(i64::MAX as u128) as i64;

//         UnixTimeUs(unix_time)
//     }
// }

pub trait EventQuery {
    fn get_query(self) -> Query<'static, Sqlite, SqliteArguments<'static>>;
}

impl EventQuery for DomainEvents {
    fn get_query(self) -> Query<'static, Sqlite, SqliteArguments<'static>> {
        match self {
            DomainEvents::Questioner(questioner_event) => questioner_event.get_query(),
        }
    }
}

pub struct EventQueue {
    events: Vec<DomainEvents>,
}

impl EventQueue {
    pub fn new() -> Self {
        let events = Vec::new();

        Self { events }
    }
}

impl EventQueue {
    pub fn append(&mut self, events: &mut Vec<DomainEvents>) {
        self.events.append(events);
    }

    pub fn push(&mut self, events: DomainEvents) {
        self.events.push(events);
    }

    pub fn take_all_events(&mut self) -> Vec<DomainEvents> {
        std::mem::take(&mut self.events)
    }
}

pub(crate) async fn save<Event>(events: Vec<Event>, queue: &Arc<Mutex<EventQueue>>)
where
    Event: Into<DomainEvents>,
{
    let mut queue = queue.lock().await;

    for event in events {
        queue.push(event.into())
    }
}

pub struct SqlxUnitOfWork {
    queue: Arc<Mutex<EventQueue>>, // TODO: Create an event bus that emits the events onto it
    pool: Arc<Pool<Sqlite>>,
    pub questioner: SqlxQuestionerRepository,
}

#[async_trait]
impl UnitOfWork for SqlxUnitOfWork {
    type Error = crate::Error;
    type QuestionerRepo = SqlxQuestionerRepository;

    async fn commit(&self) -> Result<()> {
        let events = self.queue.lock().await.take_all_events();

        let mut txn = self.pool.begin().await?;
        for event in events {
            let query = event.get_query();
            query.execute(txn.as_mut()).await?;
        }

        txn.commit().await?;

        Ok(())
    }

    fn questioner_repo(&self) -> &Self::QuestionerRepo {
        &self.questioner
    }
}
impl SqlxUnitOfWork {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        let queue = EventQueue::new();
        let queue = Arc::new(Mutex::new(queue));
        let pool = Arc::new(pool);
        Self {
            questioner: SqlxQuestionerRepository::new(Arc::clone(&pool), Arc::clone(&queue)),
            queue,
            pool,
        }
    }
}

pub async fn migrate_up() -> std::result::Result<(), sqlx::Error> {
    let db_url = &core_config().DB_URL;

    let migrator = Migrator::new(Path::new("../../migrations")).await?;

    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(db_url)
        .await?;

    migrator.run(&pool).await?;

    Ok(())
}

pub async fn establish_connection_pool() -> Result<Pool<Sqlite>> {
    let pool = establish_connection(&core_config().DB_URL).await?;

    Ok(pool)
}

pub async fn establish_connection(db_url: &String) -> Result<Pool<Sqlite>> {
    let max_connections = if cfg!(test) { 1 } else { 5 };

    let pool = SqlitePoolOptions::new()
        .max_connections(max_connections)
        .connect(db_url)
        .await?;

    Ok(pool)
}

pub async fn get_unit_of_work(test: bool) -> Result<SqlxUnitOfWork> {
    load_env_file(test);
    let pool = establish_connection_pool().await?;

    Ok(SqlxUnitOfWork::new(pool))
}

#[cfg(test)]
mod tests {
    use application::{QuestionerCommand, Repository, TaskDto, UnitOfWork};
    use domain::{
        questioner::{ExpressionStr, QuestionerId},
        DateTime, Duration,
    };
    use sqlx::SqlitePool;

    use crate::SqlxUnitOfWork;

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn can_create(pool: SqlitePool) {
        let uow = SqlxUnitOfWork::new(pool);
        create(&uow).await.unwrap();
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn can_get_by_id(pool: SqlitePool) {
        let uow = SqlxUnitOfWork::new(pool);
        let id = create(&uow).await.unwrap();

        let entity = uow.questioner_repo().get_by_id(id).await.unwrap();

        assert_ne!(None, entity);
    }

    async fn create(uow: &impl UnitOfWork) -> Result<QuestionerId, application::Error> {
        let id = QuestionerId::new();
        let command = QuestionerCommand::Create {
            id,
            allotted_time: Duration::from_seconds(45),
            tasks: vec![TaskDto {
                expression: ExpressionStr::parse("5+4-1"),
                answered: 8,
                answer_correct: true,
                answer_duration: Duration::from_seconds(2),
                answered_at: DateTime::now(),
            }],
        };

        command.handle(uow).await?;

        Ok(id)
    }
}
