#![allow(unused)]
#![allow(dead_code)]

use chrono::{DateTime, Local, NaiveDate};
use serde::{Deserialize, Serialize};
use crate::timestamp::Timestamp;
use crate::value::Value;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Args(Vec<Value>);

impl Args {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }
    pub fn arg<T:ValueConvertible>(mut self, data: T) -> Self {
        self.0.push(data.to_value());
        self
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn get(&self, index: usize) -> Option<&Value> {
        self.0.get(index)
    }
    pub fn iter(&self) -> std::slice::Iter<'_, Value> {
        self.0.iter()
    }
}

pub trait ValueConvertible {
    fn to_value(&self) -> Value;
}

/********************************************************************
*                                                                   *
*  V a l u e C o n v e r t i b l e   I m p l e m e n t a t i o n s  *
*                                                                   *
********************************************************************/

//------- Numbers -----------------------------------------

impl ValueConvertible for i8 {
    fn to_value(&self) -> Value {
        (*self as i64).into()
    }
}
impl ValueConvertible for u8 {
    fn to_value(&self) -> Value {
        (*self as i64).into()
    }
}
impl ValueConvertible for i16 {
    fn to_value(&self) -> Value {
        (*self as i64).into()
    }
}
impl ValueConvertible for u16 {
    fn to_value(&self) -> Value {
        (*self as i64).into()
    }
}
impl ValueConvertible for i32 {
    fn to_value(&self) -> Value {
        (*self as i64).into()
    }
}
impl ValueConvertible for u32 {
    fn to_value(&self) -> Value {
        (*self as i64).into()
    }
}
impl ValueConvertible for i64 {
    fn to_value(&self) -> Value {
        (*self).into()
    }
}
impl ValueConvertible for f32 {
    fn to_value(&self) -> Value {
        (*self as f64).into()
    }
}
impl ValueConvertible for f64 {
    fn to_value(&self) -> Value {
        (*self).into()
    }
}

//------- Containers  -------------------------------------

impl ValueConvertible for &str {
    fn to_value(&self) -> Value {
        (*self).into()
    }
}
impl ValueConvertible for &String {
    fn to_value(&self) -> Value {
        (*self).clone().into()
    }
}
impl ValueConvertible for &[u8] {
    fn to_value(&self) -> Value {
        (*self).into()
    }
}
impl ValueConvertible for &Vec<u8> {
    fn to_value(&self) -> Value {
        (*self).clone().into()
    }
}

//------- Optional numbers --------------------------------

impl ValueConvertible for &Option<i8> {
    fn to_value(&self) -> Value {
        match self {
            Some(v) => Value::from(*v as i64),
            None => Value::from(()),
        }
    }
}
impl ValueConvertible for &Option<u8> {
    fn to_value(&self) -> Value {
        match self {
            Some(v) => Value::from(*v as i64),
            None => Value::from(()),
        }
    }
}
impl ValueConvertible for &Option<i16> {
    fn to_value(&self) -> Value {
        match self {
            Some(v) => Value::from(*v as i64),
            None => Value::from(()),
        }
    }
}
impl ValueConvertible for &Option<u16> {
    fn to_value(&self) -> Value {
        match self {
            Some(v) => Value::from(*v as i64),
            None => Value::from(()),
        }   
    }
}
impl ValueConvertible for &Option<i32> {
    fn to_value(&self) -> Value {
        match self {
            Some(v) => Value::from(*v as i64),
            None => Value::from(()),
        }
    }
}
impl ValueConvertible for &Option<u32> {
    fn to_value(&self) -> Value {
        match self {
            Some(v) => Value::from(*v as i64),
            None => Value::from(()),       
        }
    }
}
impl ValueConvertible for &Option<i64> {
    fn to_value(&self) -> Value {
        match self {
            Some(v) => Value::from(*v),
            None => Value::from(()),
        }
    }
}
impl ValueConvertible for &Option<f32> {
    fn to_value(&self) -> Value {
        match self {
            Some(v) => Value::from(*v as f64),
            None => Value::from(()),       
        }
    }
}
impl ValueConvertible for &Option<f64> {
    fn to_value(&self) -> Value {
        match self {
            Some(v) => Value::from(*v),
            None => Value::from(()),
        }
    }
}

//------- Optional containers  ----------------------------

impl ValueConvertible for &Option<String> {
    fn to_value(&self) -> Value {
        match self {
            Some(v) => Value::from(v.clone()),
            None => Value::from(()),
        }
    }
}
impl ValueConvertible for &Option<Vec<u8>> {
    fn to_value(&self) -> Value {
        match self {
            Some(v) => Value::from(v.clone()),
            None => Value::from(()),
        }
    }
}

//------- NaiveDate ---------------------------------------

impl ValueConvertible for NaiveDate {
    fn to_value(&self) -> Value {
        (*self).into()
    } 
}
impl ValueConvertible for &Option<NaiveDate> {
    fn to_value(&self) -> Value {
        match self {
            Some(v) => Value::from(*v),
            None => Value::from(()),
        }
    }  
}

impl ValueConvertible for DateTime<Local> {
    fn to_value(&self) -> Value {
        (*self).into()
    }
}
impl ValueConvertible for &Option<DateTime<Local>> {
    fn to_value(&self) -> Value {
        match self {
            Some(v) => Value::from(*v),
            None => Value::from(()),
        }
    }
}

//------- Timestamps --------------------------------------

impl ValueConvertible for Timestamp {
    fn to_value(&self) -> Value {
        self.value().into()
    }
}
impl ValueConvertible for &Option<Timestamp> {
    fn to_value(&self) -> Value {
        match self {
            Some(v) => Value::from(v.value()),
            None => Value::from(()),
        }
    }
}

//------- Null --------------------------------------------
impl ValueConvertible for () {
    fn to_value(&self) -> Value {
        Value::Null
    }
}
