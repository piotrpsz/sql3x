use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, TimeZone};
use crate::value::Value;

impl TryFrom<Value> for i16 {
    type Error =&'static str;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v {
            Value::I64(i) => Ok(i as i16),
            _ => Err("Value is not i32".into())
        }
    }
}

impl TryFrom<Value> for i32 {
    type Error =&'static str;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v {
            Value::I64(i) => Ok(i as i32),
            _ => Err("Value is not i32".into())
        }
    }
}

impl TryFrom<Value> for i64 {
    type Error = &'static str;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v {
            Value::I64(i) => Ok(i),
            _ => Err("Value is not i64".into())
        }
    }
}
impl TryFrom<Value> for f64 {
    type Error = &'static str;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v {
            Value::F64(i) => Ok(i),
            _ => Err("Value is not f64".into())
        }
    }
}

impl TryFrom<Value> for String {
    type Error = &'static str;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v {
            Value::Text(i) => Ok(i),
            _ => Err("Value is not text".into())
        }
    }
}

impl TryFrom<Value> for Vec<u8> {
    type Error = &'static str;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        println!("try from for Vec<u8>");
        match v {
            Value::Blob(i) => Ok(i),
            _ => Err("Value is not text".into())
        }
    }
}

impl TryFrom<Value> for NaiveDate {
    type Error = &'static str;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v {
            Value::Text(text) => Ok(NaiveDate::parse_from_str(text.as_str(), "%Y-%m-%d").unwrap()),
            _ => Err("Value is not text".into())
        }
    }
}

impl TryFrom<Value> for DateTime<Local> {
    type Error = &'static str;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v {
            Value::Text(text) => {
              let naive = NaiveDateTime::parse_from_str(text.as_str(), "%Y-%m-%d %H:%M:%S").unwrap();
              let local: DateTime<Local> = Local.from_local_datetime(&naive).unwrap(); 
                  //Local.from_utc_datetime(&naive);
              Ok(local)
            },
            _ => Err("Value is not text".into())
        }
    }   
}
