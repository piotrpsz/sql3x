use std::option::Option;
use std::fmt::Display;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use crate::error::{Result};
use crate::timestamp::Timestamp;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Null,
    I64(i64),
    F64(f64),
    Text(String),
    Blob(Vec<u8>),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::I64(i) => return write!(f, "i64({})", i),
            Value::F64(fv) => return write!(f, "f64({})", fv),
            Value::Text(t) => return write!(f, "text({})", t),
            Value::Blob(b) => return write!(f, "blob({:?})", b),
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
    
    pub fn get<T> (self) -> Option<T>
        where T:TryFrom<Value>
    {
        match T::try_from(self) {
            Ok(v) => Some(v),
            Err(_) => None
        }
    }
}

/// Convert integer to Value.
impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Value::I64(i)
    }
}
impl From<Option<i64>> for Value {
    fn from(i: Option<i64>) -> Self {
        match i {
            Some(i) => Value::from(i),
            None => Value::from(())
        }
    }   
}

/// Convert float to Value.
impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Value::F64(v)
    }
}
impl From<Option<f64>> for Value {
    fn from(v: Option<f64>) -> Self {
        match v {
            Some(v) => Value::from(v),
            None => Value::from(())
        }
    }   
}

/// Convert string to Value.
impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::Text(s)
    }
}
impl From<Option<String>> for Value {
    fn from(s: Option<String>) -> Self {
        match s {
            Some(s) => Value::from(s),
            None => Value::from(())
        }
    }   
}
impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::Text(s.to_string())
    }
}
impl From<Option<&str>> for Value {
    fn from(s: Option<&str>) -> Self {
        match s {
            Some(s) => Value::from(s),
            None => Value::from(())
        }
    }   
}

/// Convert bytes to Value.
impl From<Vec<u8>> for Value {
    fn from(v: Vec<u8>) -> Self {
        Value::Blob(v)
    }
}
impl From<Option<Vec<u8>>> for Value {
    fn from(v: Option<Vec<u8>>) -> Self {
        match v {
            Some(v) => Value::from(v),
            None => Value::from(())
        }
    }   
}
impl From<&[u8]> for Value {
    fn from(v: &[u8]) -> Self {
        Value::Blob(v.to_vec())
    }
}
impl From<Option<&[u8]>> for Value {
    fn from(v: Option<&[u8]>) -> Self {
        match v {
            Some(v) => Value::from(v),
            None => Value::from(())
        }
    }   
}

/// Convert DateTime to Value.
impl From<DateTime<Local>> for Value {
    fn from(d: DateTime<Local>) -> Self {
        Value::Text(d.format("%Y-%m-%d %H:%M:%S").to_string())
    }
}
impl From<Option<DateTime<Local>>> for Value {
    fn from(d: Option<DateTime<Local>>) -> Self {
        match d {
            Some(d) => Value::from(d),
            None => Value::from(())
        }   
    }
}

/// Convert Timestamp to Value.
impl From<Timestamp> for Value {
    fn from(t: Timestamp) -> Self {
        Value::I64(t.value())
    }
}
impl From<Option<Timestamp>> for Value {
    fn from(t: Option<Timestamp>) -> Self {
        match t {
            Some(t) => Value::from(t),
            None => Value::from(())
        }   
    }
}

/// Convert () to Value.
impl From<()> for Value {
    fn from(_: ()) -> Self {
        Value::Null
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_from_json_test() {
        println!("--------------------------------------------");
        // 
        // let v = Value::Null;
        // println!("{}", v);
        // let json = v.to_json().unwrap();
        // assert_eq!(v, Value::from_json(&json).unwrap());
        // 
        // let v = Value::I64(1928);
        // println!("{}", v);
        // let json = v.to_json().unwrap();
        // assert_eq!(v, Value::from_json(&json).unwrap());
        // 
        // let v = Value::F64(1928.0);
        // println!("{}", v);
        // let json = v.to_json().unwrap();
        // assert_eq!(v, Value::from_json(&json).unwrap());
        // 
        // let v = Value::Text("hello".to_string());
        // println!("{}", v);
        // let json = v.to_json().unwrap();
        // assert_eq!(v, Value::from_json(&json).unwrap());
        // 
        // let v = Value::Blob(vec![1, 2, 3]);
        // println!("{}", v);
        // let json = v.to_json().unwrap();
        // assert_eq!(v, Value::from_json(&json).unwrap());
        // 
        // let v = Local::now();
        // let v = Value::Text(v.format("%Y-%m-%d %H:%M:%S").to_string());
        // println!("{}", v);
        // let json = v.to_json().unwrap();
        // assert_eq!(v, Value::from_json(&json).unwrap());
        
        let v = Value::from(12);
        println!("{:?}", v.get::<i64>());
        
        let v = Value::from(1928.56);
        println!("{:?}", v.get::<f64>());
        
        let v = Value::from("hello".to_string());
        println!("{:?}", v.get::<String>());
        
        let v = Value::from(vec![1, 2, 3]);
        println!("{:?}", v.get::<Vec<u8>>());
    }
}