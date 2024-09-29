use std::sync::Arc;

use application::{QuestionerRepository, Repository};
use async_trait::async_trait;
use domain::{
    questioner::{ExpressionStr, Questioner, QuestionerEvent, QuestionerId, Task},
    Aggregate, DateTime, Duration,
};
use sqlx::{prelude::FromRow, types::Uuid, Pool, Sqlite};
use tokio::sync::Mutex;

use crate::{save, EventQuery, EventQueue, Result};

impl EventQuery for QuestionerEvent {
    fn get_query(
        self,
    ) -> sqlx::query::Query<'static, Sqlite, sqlx::sqlite::SqliteArguments<'static>> {
        match self {
            QuestionerEvent::Created {
                id,
                allotted_time,
                correct_answers,
            } => sqlx::query(
                "INSERT INTO Questioners (id, allotted_time, correct_answers) VALUES (?, ?, ?)",
            )
            .bind(id.get_value())
            .bind(allotted_time.unix_timestamp())
            .bind(correct_answers.get_value()),
            QuestionerEvent::TaskAnswered {
                questioner_id,
                expression,
                answered,
                answer_correct,
                answer_duration,
                answered_at,
            } => sqlx::query(
                "INSERT INTO Tasks (questioner_id, expression, answered, answer_correct, answer_duration, answered_at) VALUES (?, ?, ?, ?, ?, ?)"
            )
                .bind(questioner_id.get_value())
                .bind(expression.get_value())
                .bind(answered)
                .bind(answer_correct)
                .bind(answer_duration.unix_timestamp())
                .bind(answered_at.unix_timestamp())
            ,
        }
    }
}

pub struct SqlxQuestionerRepository {
    pool: Arc<Pool<Sqlite>>,
    queue: Arc<Mutex<EventQueue>>,
}

impl SqlxQuestionerRepository {
    pub(crate) fn new(pool: Arc<Pool<Sqlite>>, queue: Arc<Mutex<EventQueue>>) -> Self {
        Self { pool, queue }
    }
}

impl QuestionerRepository for SqlxQuestionerRepository {}

#[async_trait]
impl Repository<Questioner> for SqlxQuestionerRepository {
    type Error = crate::Error;

    async fn generate_id(&self) -> QuestionerId {
        QuestionerId::new()
    }
    async fn get_by_id(&self, id: QuestionerId) -> Result<Option<Questioner>> {
        let questioner: Option<QuestionerDto> =
            sqlx::query_as("SELECT id, allotted_time FROM Questioners WHERE id = ?")
                .bind(id.get_value())
                .fetch_optional(self.pool.as_ref())
                .await?;

        Ok(if let Some(questioner) = questioner {
            let tasks: Vec<TaskDto> = sqlx::query_as("SELECT * FROM Tasks WHERE questioner_id = ?")
                .bind(id.get_value())
                .fetch_all(self.pool.as_ref())
                .await?;

            let tasks = tasks
                .iter()
                .map(|x| {
                    Task::new(
                        ExpressionStr::parse(&x.expression),
                        x.answered,
                        x.answer_correct,
                        Duration::from_seconds(x.answer_duration),
                        DateTime::parse(x.answered_at).unwrap(),
                    )
                })
                .collect();

            Some(Questioner::new(
                QuestionerId::parse(&questioner.id.to_string()).unwrap(),
                Duration::from_seconds(questioner.allotted_time),
                tasks,
            ))
        } else {
            None
        })
    }

    async fn save(&self, entity: Questioner) -> Result<()> {
        save(entity.get_events(), &self.queue).await;
        Ok(())
    }

    async fn delete(&self, _entity: Questioner) -> Result<Questioner> {
        todo!()
    }
}

#[derive(FromRow)]
struct QuestionerDto {
    id: Uuid,
    allotted_time: i64,
}

#[derive(FromRow)]
struct TaskDto {
    expression: String,

    answered: i32,
    answer_correct: bool,
    answer_duration: i64,
    answered_at: i64,
}
