use std::fmt::Display;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use crate::error::{Result};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Null,
    I64(i64),
    F64(f64),
    Text(String),
    Blob(Vec<u8>),
    Date(DateTime<Local>),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::I64(i) => return write!(f, "i64({})", i),
            Value::F64(fv) => return write!(f, "f64({})", fv),
            Value::Text(t) => return write!(f, "text({})", t),
            Value::Blob(b) => return write!(f, "blob({:?})", b),
            Value::Date(d) => return write!(f, "date({})", d.format("%Y-%m-%d %H:%M:%S")),
            _ => write!(f, "null")
        }
    }
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


impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Value::I64(i)
    }
}
impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Value::F64(v)
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::Text(s)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::Text(s.to_string())
    }
}

impl From<Vec<u8>> for Value {
    fn from(v: Vec<u8>) -> Self {
        Value::Blob(v)
    }
}
impl From<&[u8]> for Value {
    fn from(v: &[u8]) -> Self {
        Value::Blob(v.to_vec())
    }
}
impl From<DateTime<Local>> for Value {
    fn from(d: DateTime<Local>) -> Self {
        Value::Date(d)
    }   
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_from_json_test() {
        println!("--------------------------------------------");
        
        let v = Value::Null;
        println!("{}", v);
        let json = v.to_json().unwrap();
        assert_eq!(v, Value::from_json(&json).unwrap());
        
        let v = Value::I64(1928);
        println!("{}", v);
        let json = v.to_json().unwrap();
        assert_eq!(v, Value::from_json(&json).unwrap());
        
        let v = Value::F64(1928.0);
        println!("{}", v);
        let json = v.to_json().unwrap();
        assert_eq!(v, Value::from_json(&json).unwrap());
        
        let v = Value::Text("hello".to_string());
        println!("{}", v);
        let json = v.to_json().unwrap();
        assert_eq!(v, Value::from_json(&json).unwrap());
        
        let v = Value::Blob(vec![1, 2, 3]);
        println!("{}", v);
        let json = v.to_json().unwrap();
        assert_eq!(v, Value::from_json(&json).unwrap());

        let v = Value::Date(Local::now());
        println!("{}", v);
        let json = v.to_json().unwrap();
        assert_eq!(v, Value::from_json(&json).unwrap());
    }
}