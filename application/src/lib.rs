use std::fmt::Debug;

use async_trait::async_trait;
use domain::{
    questioner::{ExpressionStr, Questioner, QuestionerId, Task},
    Aggregate, DateTime, Duration,
};
use serde::{Deserialize, Serialize};

pub mod env;

#[derive(Debug)]
pub enum Error {
    Persistence(String),
}

#[async_trait]
pub trait UnitOfWork {
    type Error: Debug;
    type QuestionerRepo: QuestionerRepository<Error = Self::Error>;

    fn questioner_repo(&self) -> &Self::QuestionerRepo;

    async fn commit(&self) -> Result<(), Self::Error>;
}

#[async_trait]
pub trait Repository<Entity: Aggregate> {
    type Error;

    async fn generate_id(&self) -> Entity::Id;
    async fn get_by_id(&self, id: Entity::Id) -> Result<Option<Entity>, Self::Error>;

    async fn save(&self, entity: Entity) -> Result<(), Self::Error>;
    async fn delete(&self, entity: Entity) -> Result<Entity, Self::Error>;
}

pub struct QuestionerStats {
    pub high_score: u32,
    pub daily_streak: u32,
    pub previous_score: u32,
}

#[async_trait]
pub trait QuestionerRepository: Repository<Questioner> {
    async fn get_stats(&self) -> Result<QuestionerStats, Self::Error>;
}

#[derive(Serialize, Deserialize)]
pub struct TaskDto {
    pub expression: ExpressionStr,

    pub answered: i32,
    pub answer_correct: bool,
    pub answer_duration: Duration,
    pub answered_at: DateTime,
}

impl From<Task> for TaskDto {
    fn from(value: Task) -> Self {
        TaskDto {
            expression: value.expression().clone(),
            answered: value.answered(),
            answer_correct: value.answer_correct(),
            answer_duration: value.answer_duration(),
            answered_at: value.answered_at(),
        }
    }
}

pub enum QuestionerCommand {
    Create {
        id: QuestionerId,
        allotted_time: Duration,
        tasks: Vec<TaskDto>,
    },
}

impl QuestionerCommand {
    pub async fn handle(self, uow: &impl UnitOfWork) -> Result<(), Error> {
        match self {
            QuestionerCommand::Create {
                id,
                allotted_time,
                tasks,
            } => {
                let tasks = tasks
                    .iter()
                    .map(|x| {
                        Task::new(
                            x.expression.clone(),
                            x.answered,
                            x.answer_correct,
                            x.answer_duration,
                            x.answered_at,
                        )
                    })
                    .collect();
                let questioner = Questioner::create(id, allotted_time, tasks);

                uow.questioner_repo()
                    .save(questioner)
                    .await
                    .map_err(|err| Error::Persistence(format!("{:?}", err)))?;
            }
        }

        uow.commit()
            .await
            .map_err(|err| Error::Persistence(format!("{:?}", err)))?;

        Ok(())
    }
}
