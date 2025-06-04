use crate::value::Value;

pub struct Field {
    pub name: String,
    pub value: Value
}

impl Field {
    pub fn new(name: String, value: Value) -> Field {
        Field { name, value }   
    }
}
