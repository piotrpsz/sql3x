use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use crate::Result;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    I64(i64),
    F64(f64),
    Text(String),
    Blob(Vec<u8>),
    Date(DateTime<Local>),
}

impl Value {
    pub fn to_json(&self) -> Result<String> {
        match serde_json::to_string(self) {
            Ok(json) => Ok(json),
            Err(e) => Err(e.into())
        }
    }

    pub fn from_json(json: &str) -> Result<Self> {
        match serde_json::from_str(json) {
            Ok(value) => Ok(value),
            Err(e) => Err(e.into())
        }
    }
    
}