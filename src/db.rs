#![allow(unused)]
use std::ffi::{CStr, CString};
use std::fs;
use std::io::ErrorKind;
use std::ptr::{null_mut, null};
use std::io::ErrorKind::Other;
use crate::error::Result;
use crate::error::Error;
use sqlite3_sys::{sqlite3, sqlite3_close, sqlite3_errcode, sqlite3_errmsg, sqlite3_exec, sqlite3_last_insert_rowid, sqlite3_libversion, sqlite3_open_v2, sqlite3_shutdown, SQLITE_OK, SQLITE_DONE, SQLITE_OPEN_CREATE, SQLITE_OPEN_READONLY, SQLITE_OPEN_READWRITE};
use log::error;
use crate::args::Args;
use crate::query::Query;
use crate::QueryResult;
use crate::stmt::Stmt;

const IN_MEMORY: &str = ":memory:";

pub struct SQLite {
    db: *mut sqlite3,
    path: String
}

impl SQLite {
    /// Create a new database object.
    /// Default is in memory.
    pub fn new() -> SQLite {
        SQLite {
            db: std::ptr::null_mut(),
            path: IN_MEMORY.into()       
        }
    }
    
    /// Database object for a database file.
    pub fn dbf(mut self, path: &str) -> Self {
        self.path = path.into();
        self
    }

    /// Close the database.
    pub fn close(&mut self) -> Result<()> {
        // if the database is already closed (or not opened), it is OK.
        if self.db == null_mut() {
            return Ok(());       
        }
        
        unsafe {
            match sqlite3_close(self.db) {
                SQLITE_OK => {
                    self.db = null_mut();
                    Ok(())
                },
                _ => Err(self.error())
            }
        }
    }
    
    fn cstr(&self, s: &str) -> CString {
        CString::new(s.as_bytes()).unwrap()
    }

    /// Open a database.
    pub fn open(&mut self, read_only: bool) -> Result<()>{
        if self.db != std::ptr::null_mut() {
            return Err("database already open".into());
        }
        let flags = match read_only {
            true => SQLITE_OPEN_READONLY,
            _ => SQLITE_OPEN_READWRITE
        };
        unsafe {
            let path = self.cstr(self.path.as_str());
            let stat = sqlite3_open_v2(path.as_ptr(), &mut self.db, flags, null());
            match stat {
                SQLITE_OK => Ok(()),
                _ => {
                    self.close()?;
                    Err(self.error())
                }
            }
        }
    }

    /// Create a database.
    pub fn create<F>(&mut self, overwrite: bool, init: F) -> Result<()> 
        where F: Fn(&mut SQLite) -> Result<()>
    {
        if self.db != null_mut() {
            return Err("database already open".into());
        }
        if self.path != IN_MEMORY {
            self.remove_file(self.path.as_str(), overwrite)?;
        }
        
        unsafe {
            let flags = SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE;
            let path = self.cstr(self.path.as_str());
            let stat = sqlite3_open_v2(path.as_ptr(), &mut self.db, flags, null());
            match stat {
                SQLITE_OK => Ok(init(self)?),
                _ => {
                    self.close()?;
                    Err(self.error())
                }
            }       
        }
    }
    
    /// Open a database or create it if it does not exist.
    pub fn open_or_create<F>(&mut self, init: F) -> Result<()> 
        where F: Fn(&mut SQLite) -> Result<()>
    {
        if let Ok(()) = self.open(true) {
            println!("database opened: {}", self.path);
            return Ok(());    
        }
        self.create(true, init)?;
        println!("database created: {}", self.path);
        Ok(())
    }
    
    fn remove_file(&self, path: &str, overwrite: bool) -> Result<()> {
        match fs::exists(path) {
            Ok(true) => {
                // file exists
                if !overwrite {
                    return Err("file exists".into());
                }
                // try to remove the file
                match fs::remove_file(path) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e.into())   
                }
            },
            Err(e) => {
                if e.kind() == ErrorKind::NotFound {
                    return Ok(());
                }
                Err(e.into())
            },
            _ => Ok(())
        }
    }
    
    /// Execute a query without arguments.
    pub fn exec_command(&mut self, cmd: &str) -> Result<()> {
        self.database_opened()?;
        
        unsafe {
            let sql = self.cstr(cmd);
            let stat = sqlite3_exec(self.db, sql.as_ptr(), None, null_mut(), null_mut());
            match stat {
                SQLITE_OK => Ok(()),
                _ => Err(self.error())
            }
        }
    }
    
    
    /// Execute a query.
    pub fn exec(&mut self, query: Query) -> Result<()> {
        self.database_opened()?;
        let mut stmt = Stmt::for_command(self.db, query.cmd.as_str())?;
        if query.are_arguments() {
            stmt.bind(query.args)?;
        }
        match stmt.step() {
            SQLITE_OK | SQLITE_DONE => Ok(()),
            _ => Err(self.error())
        }
    }
    pub fn exec_for_query(&mut self, query: &Query) -> Result<()> {
        self.database_opened()?;
        let mut stmt = Stmt::for_command(self.db, query.cmd.as_str())?;
        if query.are_arguments() {
            stmt.bind(query.args.clone())?;
        }
        match stmt.step() {
            SQLITE_OK | SQLITE_DONE => Ok(()),
            _ => Err(self.error())
        }
    }

    /// Execute a query for inserting data.
    pub(crate) fn insert_for_query(&mut self, query: &Query) -> Result<i64> {
        self.database_opened()?;
        self.exec_for_query(&query)?;
        Ok(self.last_inserted_id())
    }
    /// Execute a query for inserting data.
    pub fn insert(&mut self, query: Query) -> Result<i64> {
        self.database_opened()?;
        self.exec(query)?;
        Ok(self.last_inserted_id())
    }

    pub(crate) fn update_for_query(&mut self, query: &Query) -> Result<()> {
        self.database_opened()?;
        self.exec_for_query(&query)
    }
    pub fn update(&mut self, query: Query) -> Result<()> {
        self.database_opened()?;
        self.exec(query)
    }

    /// Execute a query for updating data.
    pub fn select(&mut self, query: Query) -> Result<QueryResult> {
        self.database_opened()?;
        let mut stmt = Stmt::for_command(self.db, query.cmd.as_str())?;
        if query.are_arguments() {
            stmt.bind(query.args)?;
        }
        stmt.fetch_result()
    }
    
    /// Check if database is opened.
    fn database_opened(&self) -> Result<()> {
        if self.db == null_mut() {
            return Err("database not opened".into());
        }
        Ok(())
    }
    
    pub fn error(&mut self) -> Error {
        Error{ code: self.err_code(), message: self.err_string(), kind: Some(Other)}
    }

    /// Get the error code from sqlite3.
    pub fn err_code(&self) -> i32 {
        unsafe {
            let code = sqlite3_errcode(self.db);
            code as i32
        }
    }

    /// Get the error message from sqlite3.
    pub(crate) fn err_string(&self) -> String {
        unsafe {
            let text = CStr::from_ptr(sqlite3_errmsg(self.db));
            text.to_string_lossy().into_owned()
        }
    }

    /// Get the version of the SQLite library.
    pub fn version() -> String {
        unsafe { CStr::from_ptr(sqlite3_libversion()).to_string_lossy().into_owned() }
    }

    /// Get the last inserted row id.
    fn last_inserted_id(&self) -> i64 {
        unsafe { sqlite3_last_insert_rowid(self.db) }
    }
    
}

impl Drop for SQLite {
    fn drop(&mut self) {
        unsafe {
            match self.close() {
                Err(e) => error!("failed to close database: {:?}", e),
                _ => ()
            }
        }
    }
}
