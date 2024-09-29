#![allow(dead_code)]

use questioner::QuestionerEvent;
use serde::{Deserialize, Deserializer, Serialize};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Duration(time::Duration);
impl Duration {
    pub fn from_seconds(seconds: i64) -> Self {
        Duration(seconds.seconds())
    }

    pub fn unix_timestamp(self) -> i64 {
        self.0.whole_seconds()
    }
}

impl Serialize for Duration {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i64(self.unix_timestamp())
    }
}

impl<'de> Deserialize<'de> for Duration {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let seconds = i64::deserialize(deserializer)?;

        Ok(Duration::from_seconds(seconds))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    pub fn now() -> Self {
        DateTime(time::OffsetDateTime::now_utc())
    }
}

impl Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i64(self.unix_timestamp())
    }
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let seconds = i64::deserialize(deserializer)?;
        let value = DateTime::parse(seconds);

        if value.is_none() {
            Err(serde::de::Error::custom("Failed to parse the value"))
        } else {
            Ok(value.unwrap())
        }
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
