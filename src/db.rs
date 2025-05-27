use std::ffi::CStr;
use sqlite3_sys::{sqlite3, sqlite3_last_insert_rowid, sqlite3_libversion};

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
            path: ":memory:".into()
        }
    }
    
    /// Database object for a database file.
    pub fn dbf(mut self, path: &str) -> Self {
        self.path = path.into();
        self
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