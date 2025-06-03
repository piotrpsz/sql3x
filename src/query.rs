#![allow(unused)]
use crate::args::{Args, ValueConvertible};
use crate::db::SQLite;
use crate::value::Value;
use crate::error::Result;
use crate::QueryResult;

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
    pub fn with_args(query: &str, args: Args) -> Self {
        Self {
            cmd: query.to_string(),
            args
        }
    }
    
    pub fn arg<T:ValueConvertible>(mut self, arg: T) -> Self {
        self.args = self.args.arg(arg);
        self
    }
    
    pub fn are_arguments(&self) -> bool {
        !self.args.is_empty()
    }
    
    pub fn insert(&self, sq: &mut SQLite) -> Result<i64> {
        self.validate()?;
        sq.insert_for_query(self)
    }
    pub fn update(&self, sq: &mut SQLite) -> Result<()> {
        self.validate()?;       
        sq.update_for_query(self)
    }
    pub fn select(&self, sq: &mut SQLite) -> Result<QueryResult> {
        self.validate()?;
        sq.select_for_query(self)
    }
    pub fn delete(&self, sq: &mut SQLite) -> Result<()> {
        self.validate()?;
        sq.delete_for_query(self)
    }
    
    fn validate(&self) -> Result<()> {
        let placeholder_number = self.cmd
            .bytes()
            .filter(|c| *c == b'?')
            .count();
        
        if placeholder_number != self.args.len() {
            let message = format!("query not valid: invalid number of arguments. Expected: {}, got: {}", placeholder_number, self.args.len());
            return Err(message.as_str().into());       
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_from_json_test() {
        let query = Query::new("SELECT * FROM users WHERE id=? and name=? and pi=?")
            .arg(1)
            .arg("Piotr")
            .arg(3.54);
        println!("{query:?}");
    }
}