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
        match v {
            Value::Blob(i) => Ok(i),
            _ => Err("Value is not text".into())
        }
    }
}
