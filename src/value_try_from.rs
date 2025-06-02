use chrono::{
    DateTime,
    Local,
    NaiveDate,
    NaiveDateTime,
    TimeZone
};
use crate::value::Value;

const INVALID_VALUE_TYPE: &str = "invalid value type";

impl TryFrom<Value> for i8 {
    type Error =&'static str;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v {
            Value::I64(iv) => Ok(iv as i8),
            _ => Err(INVALID_VALUE_TYPE)
        }
    }
}

impl TryFrom<Value> for u8 {
    type Error =&'static str;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v {
            Value::I64(iv) => Ok(iv as u8),
            _ => Err(INVALID_VALUE_TYPE)
        }
    }
}

impl TryFrom<Value> for i16 {
    type Error =&'static str;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v {
            Value::I64(iv) => Ok(iv as i16),
            _ => Err(INVALID_VALUE_TYPE)
        }
    }
}

impl TryFrom<Value> for u16 {
    type Error =&'static str;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v {
            Value::I64(iv) => Ok(iv as u16),
            _ => Err(INVALID_VALUE_TYPE)
        }
    }
}

impl TryFrom<Value> for i32 {
    type Error =&'static str;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v {
            Value::I64(iv) => Ok(iv as i32),
            _ => Err(INVALID_VALUE_TYPE)
        }
    }
}

impl TryFrom<Value> for u32 {
    type Error =&'static str;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v {
            Value::I64(iv) => Ok(iv as u32),
            _ => Err(INVALID_VALUE_TYPE)
        }
    }
}

impl TryFrom<Value> for i64 {
    type Error = &'static str;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v {
            Value::I64(iv) => Ok(iv),
            _ => Err(INVALID_VALUE_TYPE)
        }
    }
}

impl TryFrom<Value> for f32 {
    type Error = &'static str;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v {
            Value::F64(fv) => Ok(fv as f32),
            _ => Err(INVALID_VALUE_TYPE)
        }
    }
}

impl TryFrom<Value> for f64 {
    type Error = &'static str;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v {
            Value::F64(fv) => Ok(fv),
            _ => Err(INVALID_VALUE_TYPE)
        }
    }
}

impl TryFrom<Value> for String {
    type Error = &'static str;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v {
            Value::Text(tv) => Ok(tv),
            _ => Err(INVALID_VALUE_TYPE)
        }
    }
}

impl TryFrom<Value> for Vec<u8> {
    type Error = &'static str;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v {
            Value::Blob(bv) => Ok(bv),
            _ => Err(INVALID_VALUE_TYPE)
        }
    }
}

impl TryFrom<Value> for NaiveDate {
    type Error = &'static str;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v {
            Value::Text(tv) => Ok(NaiveDate::parse_from_str(tv.as_str(), "%Y-%m-%d").unwrap()),
            _ => Err(INVALID_VALUE_TYPE)
        }
    }
}

impl TryFrom<Value> for DateTime<Local> {
    type Error = &'static str;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v {
            Value::Text(tv) => {
              let ndt = NaiveDateTime::parse_from_str(tv.as_str(), "%Y-%m-%d %H:%M:%S").unwrap();
              let dt: DateTime<Local> = Local.from_local_datetime(&ndt).unwrap(); 
              Ok(dt)
            },
            _ => Err(INVALID_VALUE_TYPE)
        }
    }   
}
