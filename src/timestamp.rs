#![allow(unused)]
use chrono::Local;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp(i64);

impl Timestamp {
    pub fn now() -> Self {
        Timestamp(Local::now().timestamp())
    }
    pub fn value(self) -> i64 {
        self.0
    }
    pub fn from_value(value: i64) -> Self {
        Timestamp(value)
    }
}

impl From<i64> for Timestamp {
    fn from(value: i64) -> Self {
        Timestamp(value)
    }
}
