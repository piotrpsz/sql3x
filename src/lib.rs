pub mod db;
pub mod value;
pub mod error;
pub mod timestamp;
pub mod args;
pub mod query;
pub mod stmt;

type Row = Vec<value::Value>;
type QueryResult = Vec<Row>;

#[cfg(test)]
mod tests {
    use crate::db::SQLite;
    use crate::error::Result;
    use crate::query::Query;
    use crate::QueryResult;

    static CREATE_PERSON_TABLE: &str = r#"
        CREATE TABLE person (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            first_name TEXT NOT NULL COLLATE NOCASE,
            second_name TEXT COLLATE NOCASE,
            surname TEXT NOT NULL COLLATE NOCASE,
            age INTEGER,
            date_time TEXT,
            timestamp INT,
            cof DOUBLE,
            data BLOB
        );
    "#;
    
    #[derive(Default)]
    pub struct Person {
        id : i64,
        first_name : String,
        second_name : Option<String>,
        surname : String,
        age : Option<i32>,
        date_time : Option<String>,
        timestamp : Option<i64>,
        cof : Option<f64>,
        data : Option<Vec<u8>>
    }

    impl Person {
        pub fn new(first_name: &str, surname: &str) -> Self {
            Person{ 
                first_name: first_name.to_string(), 
                surname: surname.to_string(),
                ..Default::default()}
        }
        
        pub fn insert(&mut self, sq: &mut SQLite) -> Result<()> {
            let query = Query::new("INSERT INTO person (first_name, surname, age) VALUES (?, ?, ?);")
                .add(self.first_name.as_str())
                .add(self.surname.as_str())
                .add(self.age);
            self.id = sq.insert(query)?;
            Ok(())
        }
        
        pub fn all(sq: &mut SQLite) -> Result<QueryResult> {
            let query = Query::new("SELECT * from person");
            sq.select(query)
        }
    }
    
    #[test]
    fn create_database()  {
        let mut sq = SQLite::new().dbf("C:\\Users\\piotr\\testowe.sqlite");
        let stat = sq.create(true, |sq|{
            sq.exec_command(CREATE_PERSON_TABLE)
        });
        
        let mut p1 = Person::new("Piotr", "Pszczółkowski");
        p1.age = Some(25);
        let id = p1.insert(&mut sq).unwrap();
        println!("{:?}", id);

        let mut p2 = Person::new("Robert", "Chełchowski");
        let id = p2.insert(&mut sq).unwrap();
        println!("{:?}", id);
        
        
        let result = Person::all(&mut sq).unwrap();
        result.iter().for_each(|row| {println!("{:?}", row);});
        
        assert!(stat.is_ok());
    }
}