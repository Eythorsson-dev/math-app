#![allow(dead_code)]

use questioner::QuestionerEvent;
use serde::Serialize;
use time::ext::NumericalDuration;

pub mod expression;
pub mod questioner;

#[derive(Debug)]
pub enum Error {
    WeightOutOfRange,
    ExpressionTooShort,
    OperatorMissing,
    TermMissing,
    ConstantOptionOutOfRange,
}
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq)]
pub struct Duration(time::Duration);
impl Duration {
    pub fn from_seconds(seconds: i64) -> Self {
        Duration(seconds.seconds())
    }

    pub fn unix_timestamp(self) -> i64 {
        self.0.whole_seconds()
    }
}

#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq)]
pub struct DateTime(time::OffsetDateTime);
impl DateTime {
    pub fn parse(seconds_since_unix_epoch: i64) -> Option<DateTime> {
        let timestamp = time::OffsetDateTime::from_unix_timestamp(seconds_since_unix_epoch);

        if let Ok(timestamp) = timestamp {
            Some(DateTime(timestamp))
        } else {
            None
        }
    }
    pub fn unix_timestamp(&self) -> i64 {
        self.0.unix_timestamp()
    }

    pub fn now() -> Self{
        DateTime(time::OffsetDateTime::now_utc())
    }
}

pub enum DomainEvents {
    Questioner(QuestionerEvent),
}

pub trait Aggregate {
    type Id;
    type Event: Clone;

    fn get_events(self) -> Vec<Self::Event>;
}
