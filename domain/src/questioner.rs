use serde::Serialize;
use uuid::Uuid;

use crate::{Aggregate, DateTime, DomainEvents, Duration};

#[derive(Debug, Serialize, Clone, Copy, PartialEq)]
pub struct QuestionerId(Uuid);
impl QuestionerId {
    pub fn new() -> Self {
        QuestionerId(Uuid::new_v4())
    }

    pub fn parse(value: &str) -> Option<QuestionerId> {
        if let Ok(uuid) = Uuid::parse_str(value) {
            Some(QuestionerId(uuid))
        } else {
            None
        }
    }

    pub fn get_value(self) -> Uuid {
        self.0
    }
}

#[derive(Debug, Serialize, Clone, Copy, PartialEq)]
pub struct CorrectAnswers(i32);
impl CorrectAnswers {
    pub fn get_value(&self) -> i32 {
        self.0
    }
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum QuestionerEvent {
    Created {
        id: QuestionerId,
        allotted_time: Duration,
        correct_answers: CorrectAnswers,
    },
    TaskAnswered {
        questioner_id: QuestionerId,
        expression: ExpressionStr,
        answered: i32,
        answer_correct: bool,
        answer_duration: Duration,
        answered_at: DateTime,
    },
}

impl QuestionerEvent {
    pub fn get_id(&self) -> QuestionerId {
        match *self {
            QuestionerEvent::Created { id, .. } => id,
            QuestionerEvent::TaskAnswered { questioner_id, .. } => questioner_id,
        }
    }
}
impl From<QuestionerEvent> for DomainEvents {
    fn from(value: QuestionerEvent) -> Self {
        DomainEvents::Questioner(value)
    }
}

#[derive(Debug, PartialEq)]
pub struct Questioner {
    events: Vec<QuestionerEvent>,

    id: QuestionerId,
    allotted_time: Duration,
    correct_answers: CorrectAnswers,

    tasks: Vec<Task>,
}
impl Aggregate for Questioner {
    type Id = QuestionerId;
    type Event = QuestionerEvent;

    fn get_events(self) -> Vec<Self::Event> {
        self.events
    }
}

impl Questioner {
    pub fn new(id: QuestionerId, allotted_time: Duration, tasks: Vec<Task>) -> Self {
        Self {
            events: Vec::new(),

            id,
            allotted_time,
            correct_answers: CorrectAnswers(
                tasks.iter().filter(|x| x.answer_correct).count() as i32
            ),
            tasks,
        }
    }

    pub fn create(id: QuestionerId, allotted_time: Duration, tasks: Vec<Task>) -> Self {
        let mut questioner = Questioner::new(id, allotted_time, tasks.to_vec());

        questioner.events.push(QuestionerEvent::Created {
            id,
            allotted_time,
            correct_answers: questioner.correct_answers,
        });

        for task in tasks {
            questioner.events.push(task.to_created_event(id));
        }

        questioner
    }
}

// TODO: Implement Expression parse_str and parse_str_vec so that we can replace this struct with Expression
#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct ExpressionStr(String);
impl ExpressionStr {
    pub fn parse(value: &str) -> ExpressionStr {
        ExpressionStr(value.to_owned())
    }

    pub fn get_value(self) -> String {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    expression: ExpressionStr,

    answered: i32,
    answer_correct: bool,
    answer_duration: Duration,
    answered_at: DateTime,
}
impl Task {
    pub fn new(
        expression: ExpressionStr,
        answered: i32,
        answer_correct: bool,
        answer_duration: Duration,
        answered_at: DateTime,
    ) -> Self {
        Self {
            expression,
            answered,
            answer_correct,
            answer_duration,
            answered_at,
        }
    }

    fn to_created_event(self, questioner_id: QuestionerId) -> QuestionerEvent {
        QuestionerEvent::TaskAnswered {
            questioner_id: questioner_id,
            expression: self.expression,
            answered: self.answered,
            answer_correct: self.answer_correct,
            answer_duration: self.answer_duration,
            answered_at: self.answered_at,
        }
    }
}
