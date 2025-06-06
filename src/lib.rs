#![allow(unused)]

use std::collections::HashMap;
use std::fmt;
use fmt::Debug;

pub mod db;
pub mod value;
pub mod error;
pub mod timestamp;
pub mod args;
pub mod query;
pub mod stmt;
pub mod value_try_from;
pub mod field;

pub type Row = HashMap<String, value::Value>;
pub type QueryResult = Vec<Row>;

pub mod prelude {
    pub use crate::db::SQLite;
    pub use crate::query::Query;
    pub use crate::value::Value;
    pub use crate::value_try_from;
    pub use crate::field::Field;
    pub use crate::timestamp::Timestamp;
    pub use crate::args::Args;
    pub use crate::stmt::Stmt;
    pub use crate::Row;
    pub use crate::QueryResult;
    pub use crate::error::Result;
}

#[cfg(test)]
mod tests {
    use std::fmt::{write, Debug, Display, Formatter};
    use crate::db::SQLite;
    use crate::error::Result;
    use crate::query::Query;
    use crate::{QueryResult, Row};
    use std::string::String;
    use chrono::{DateTime, Local, NaiveDate};

    static CREATE_PERSON_TABLE: &str = r#"
        CREATE TABLE person (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            first_name TEXT NOT NULL COLLATE NOCASE,
            second_name TEXT COLLATE NOCASE,
            surname TEXT NOT NULL COLLATE NOCASE,
            birthday DATE,
            now DATETIME,
            timestamp INT,
            cof DOUBLE,
            data BLOB
        );
    "#;
    
    #[derive(Default, Debug,  Clone, PartialEq, PartialOrd)]
    pub struct Person {
        id : i64,
        first_name : String,
        second_name : Option<String>,
        surname : String,
        birthday : Option<NaiveDate>,
        now : Option<DateTime<Local>>,
        timestamp : Option<i64>,
        cof : Option<f64>,
        data : Option<Vec<u8>>
    }
    
    impl Display for Person {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let text = format!("Person(\
                \n          id: {},\
                \n  first_name: {},\
                \n second_name: {},\
                \n     surname: {},\
                \n    birthday: {},\
                \n         now: {},\
                \n   timestamp: {},\
                \n         cof: {},\
                \n        data: {}\
                \n)",
                self.id,
                self.first_name,
                match self.second_name {
                    Some(ref s) => s,
                    None => "-",
                },
                self.surname,
                match self.birthday {
                    Some(v) => v.format("%Y-%m-%d").to_string(), 
                    None => "-".to_string(),
                },
                match self.now {
                    Some(v) => v.format("%Y-%m-%d %H:%M:%S").to_string(), 
                    None => "-".to_string(),
                },
                match self.timestamp {
                    Some(v) => v.to_string(),
                    None => "-".to_string(),   
                },
                match self.cof {
                    Some(v) => v.to_string(),
                    None => "-".to_string(),
                },
                match self.data.clone() {
                    Some(v) => format!("{v:?}"),
                    None => "-".to_string(),
                }
            );
            write!(f, "{text}")
        }
    }
   

    impl Person {
        pub fn new(first_name: &str, surname: &str) -> Self {
            Person{ 
                first_name: first_name.to_string(), 
                surname: surname.to_string(),
                ..Default::default()}
        }
        pub fn new_from_row(row: &Row) -> Self {
            let mut person = Person::default();
            person.id = row["id"].clone().get::<i64>().unwrap();
            person.first_name = row["first_name"].clone().get::<String>().unwrap();
            person.second_name = row["second_name"].clone().get::<String>();
            person.surname = row["surname"].clone().get::<String>().unwrap();
            person.birthday = row["birthday"].clone().get::<NaiveDate>();
            person.now = row["now"].clone().get::<DateTime<Local>>();
            person.timestamp = row["timestamp"].clone().get::<i64>();
            person.cof = row["cof"].clone().get::<f64>();
            person.data = row["data"].clone().get::<Vec<u8>>();
            person
        }

        pub fn with_id(id: i64, sq: &mut SQLite) -> Result<Option<Person>> {
            let mut result = Query::new("SELECT * from person WHERE id=?;")
                .arg(id)
                .select(sq)?;

            match result.len() {
                0 => Ok(None),
                1 => Ok(Some(Person::new_from_row(&result[0]))),
                _ => Err("with_id: too many rows".into())
            }
        }

        pub fn insert(&mut self, sq: &mut SQLite) -> Result<()> {
            let id = Query::new("INSERT INTO person (first_name, second_name, surname, birthday, now, timestamp, cof, data) VALUES (?,?,?,?,?,?,?,?);")
                .arg(&self.first_name)
                .arg(&self.second_name)
                .arg(&self.surname)
                .arg(&self.birthday)
                .arg(&self.now)
                .arg(&self.timestamp)
                .arg(&self.cof)
                .arg(&self.data)
                .insert(sq)?;
            self.id = id;
            Ok(())
        }
        
        pub fn update(&mut self, sq: &mut SQLite) -> Result<()> {
            Query::new("UPDATE person SET first_name=?, surname=?, birthday=? WHERE id=?;")
                .arg(&self.first_name)
                .arg(&self.surname)
                .arg(&self.birthday)
                .arg(self.id)
                .update(sq)
        }
        
        pub fn all(sq: &mut SQLite) -> Result<QueryResult> {
            let query = Query::new("SELECT * from person");
            sq.select(query)
        }
    }
    
    #[test]
    fn checks() {
        let type_name_of_value = std::any::type_name_of_val(&1i32);
        let type_name = std::any::type_name::<i32>();
        
        assert_eq!(type_name_of_value, type_name);
        
        println!("{:?}", std::any::type_name_of_val(&"ala".to_string()));
        println!("{:?}", std::any::type_name::<Vec<u8>>());
        println!("{:?}", std::any::type_name::<String>());
        println!("{:?}", std::any::TypeId::of::<i32>());
    }
    
    #[test]
    fn create_database()  {
        let mut sq = SQLite::new()
            .dbf("C:\\Users\\piotr\\testowe.sqlite")
            .create(true, |sq|{
                sq.exec_command(CREATE_PERSON_TABLE)
            }).unwrap();
        
        let mut p1 = Person::new("Piotr", "Pszczółkowski");
        p1.birthday = Some(NaiveDate::from_ymd_opt(1959, 10, 25).unwrap());
        p1.now = Some(Local::now());
        p1.timestamp = Some(Local::now().timestamp());
        p1.cof = Some(1.2345);
        p1.data = Some(vec![1u8, 2, 3, 4, 5]);
        let id = p1.insert(&mut sq);
        println!("{:?}", id);

        let result = Person::all(&mut sq).unwrap();
        result.iter().for_each(|row| {println!("{:?}", row);});
        
        // p1.age = None;
        // p1.update(&mut sq).unwrap();
        
        
        let mut p2 = Person::new("Robert", "Chełchowski");
        let id = p2.insert(&mut sq).unwrap();
        println!("{:?}", id);
        
        
        let result = Person::all(&mut sq).unwrap();
        result.iter().for_each(|row| {println!("{}", Person::new_from_row(row));});

        println!("-----------------------------------------------------------------------------");
        let person = Person::with_id(1, &mut sq).unwrap();
        println!("{}", person.unwrap());
        
        
        
    }
}