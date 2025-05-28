#![allow(unused)]
use crate::args::{Args, ValueConvertible};
use crate::value::Value;

#[derive(Clone, Debug, Default)]
pub struct Query {
    pub cmd: String,
    pub args: Args
}

impl Query {
    pub fn new(query: &str) -> Self {
        Self {
            cmd: query.to_string(),
            ..Default::default()
        }
    }
    
    pub fn add<T:ValueConvertible>(mut self, arg: T) -> Self {
        self.args = self.args.add(arg);
        self
    }
    
    pub fn are_arguments(&self) -> bool {
        !self.args.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_from_json_test() {
        let query = Query::new("SELECT * FROM users WHERE id=? and name=? and pi=?")
            .add(1)
            .add("Piotr")
            .add(3.14);
        println!("{:?}", query);
    }
}