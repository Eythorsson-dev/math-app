use std::fmt::Debug;

use async_trait::async_trait;
use domain::{
    questioner::{Questioner, QuestionerId, Task},
    Aggregate, Duration,
};

pub mod env;

#[derive(Debug)]
pub enum Error {
    Persistance(String),
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

pub trait QuestionerRepository: Repository<Questioner> {}

pub enum QuestionerCommand {
    Create {
        id: QuestionerId,
        allotted_time: Duration,
        tasks: Vec<Task>,
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
                let questioner = Questioner::create(id, allotted_time, tasks);

                uow.questioner_repo()
                    .save(questioner)
                    .await
                    .map_err(|err| Error::Persistance(format!("{:?}", err)))?;
            }
        }

        uow.commit()
            .await
            .map_err(|err| Error::Persistance(format!("{:?}", err)))?;

        Ok(())
    }
}
