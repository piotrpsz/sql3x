use std::ffi::{
    CStr, 
    CString, 
    c_char, 
    c_void,
};
use std::io::ErrorKind::Other;
use std::mem::transmute;
use std::ptr::copy;
use sqlite3_sys::{sqlite3, sqlite3_bind_int64, sqlite3_bind_double, sqlite3_bind_parameter_index, sqlite3_column_count, sqlite3_column_name, sqlite3_column_type, sqlite3_errcode, sqlite3_errmsg, sqlite3_finalize, sqlite3_prepare_v2, sqlite3_reset, sqlite3_step, sqlite3_stmt, SQLITE_OK, sqlite3_bind_text, sqlite3_bind_null, sqlite3_bind_blob, sqlite3_column_int64, sqlite3_column_double, sqlite3_column_text, sqlite3_column_blob, sqlite3_column_bytes, SQLITE_ROW, SQLITE_DONE};
use std::ptr::null_mut;
use crate::args::Args;
use crate::error::{Error, Result};
use crate::{Row, QueryResult};
use crate::query::Query;
use crate::value::Value;

pub struct Stmt {
    pub stmt: *mut sqlite3_stmt,
    db: *mut sqlite3
}

impl Stmt {
    pub fn for_command(db: *mut sqlite3, cmd: &str) -> Result<Stmt> {
        let mut stmt = Stmt {stmt: null_mut(), db};
        stmt.prepare(cmd)?;
        Ok(stmt)
    }
    
    pub(crate) fn prepare(&mut self, query: &str) -> Result<()> {
        unsafe {
            match sqlite3_prepare_v2(self.db, CString::new(query).unwrap().into_raw(), -1, &mut self.stmt, null_mut()) {
                SQLITE_OK => Ok(()),
                _ => Err(self.error())
            }
        }
    }
    
    pub(crate) fn reset(&mut self) -> Result<()> {
        unsafe {
            match sqlite3_reset(self.stmt) {
                SQLITE_OK => Ok(()),
                _ => Err(self.error())
            }
        }
    }
    
    pub(crate)fn finalize(&mut self) -> Result<()> {
        unsafe {
            match sqlite3_finalize(self.stmt) {
                SQLITE_OK => Ok(()),
                _ => Err(self.error())   
            }
        }
    }
    
    #[inline]
    pub(crate) fn step(&mut self) -> i32 {
        unsafe { sqlite3_step(self.stmt) }
    } 

    /// Columns count in result set
    #[inline]
    fn column_count(&self) -> i32 {
        unsafe { sqlite3_column_count(self.stmt) }
    }
    
    #[inline]
    fn column_type(&self, index: i32) -> i32 {
        unsafe { sqlite3_column_type(self.stmt, index as i32) }
    }
    
    /// Returns index of column with given name
    #[inline]
    fn column_idx_for_name(&self, name: &str) -> i32 {
        unsafe { sqlite3_bind_parameter_index(self.stmt, CString::new(name).unwrap().into_raw()) }
    }
    
    /// Returns name of column with given index
    #[inline]
    fn column_name_for_idx(&self, idx: i32) -> String {
        unsafe {
            let ptr = sqlite3_column_name(self.stmt, idx);
            String::from_utf8_lossy(CStr::from_ptr(ptr).to_bytes()).into()
        }
    }
    
    pub(crate) fn bind(&mut self, args: Args) -> Result<()> {
        args
            .iter()
            .enumerate()
            .try_for_each(|(idx, value)| self.bind_at(idx as i32, value))?;
        Ok(())
    }
    
    
    fn bind_at(&mut self, idx: i32, value: &Value) -> Result<()> {
        let idx = idx + 1;
        match value {
            Value::Null => self.bind_null(idx)?,
            Value::I64(x) => self.bind_i64(idx, *x)?,
            Value::F64(x) => self.bind_f64(idx, *x)?,
            Value::Text(x) => self.bind_text(idx, x)?,
            Value::Blob(x) => self.bind_blob(idx, x)?,
        }
        Ok(())
    }
    
    
    fn fetch_row(&self, columns: i32) -> Row {
        (0..columns)
            .into_iter()
            .map(|idx| {
                let column_type = self.column_type(idx);
                // https://www.sqlite.org/c3ref/c_blob.html
                match column_type {
                    1 => Value::from(self.get_i64(idx)),
                    2 => Value::from(self.get_f64(idx)),
                    3 => Value::from(self.get_text(idx)),
                    4 => Value::from(self.get_blob(idx)),
                    5 => Value::from(()),
                    _ => panic!("Unknown column type: {}", column_type),
                }
            })
            .collect()
    }

    pub fn fetch_result(&mut self) -> Result<QueryResult> {
        let columns = self.column_count();

        let mut result = QueryResult::new();
        while SQLITE_ROW == self.step() {
            result.push(self.fetch_row(columns));
        }
        
        match self.err_code() {
            SQLITE_OK | SQLITE_DONE => Ok(result),
            _ => Err(self.error())
        }
      
    }
    
    /********************************************************************
    *                                                                   *
    *                        B I N D I N G S                            *
    *                                                                   *
    ********************************************************************/
    
    /// Bind null value.
    fn bind_null(&mut self, idx: i32) -> Result<()> {
        unsafe {
            match sqlite3_bind_null(self.stmt, idx) {
                SQLITE_OK => Ok(()),
                _ => Err(self.error())
            }
        }
    }
    
    /// Bind value as i64.
    fn bind_i64(&mut self, idx: i32, value: i64) -> Result<()> {
        unsafe {
            match sqlite3_bind_int64(self.stmt, idx, value) {
                SQLITE_OK => Ok(()),
                _ => Err(self.error())  
            }
        }
    }

    /// Bind value as f64.
    fn bind_f64(&mut self, idx: i32, value: f64) -> Result<()> {
        unsafe {
            match sqlite3_bind_double(self.stmt, idx, value) {
                SQLITE_OK => Ok(()),
                _ => Err(self.error())
            }
        }
    }
    
    /// Bind value as text.
    fn bind_text(&mut self, idx: i32, value: &str) -> Result<()> {
        unsafe {
            let stat = sqlite3_bind_text(
                self.stmt,
                idx, 
                value.as_ptr() as *const c_char,
                value.len() as i32,
                transmute(!0 as *const c_void));
            
            match stat {
                SQLITE_OK => Ok(()),
                _ => Err(self.error())
            }
        }
    }
    
    fn bind_blob(&mut self, idx: i32, value: &[u8]) -> Result<()> {
        unsafe {
            let stat = sqlite3_bind_blob(
                self.stmt,
                idx,
                value.as_ptr() as *const c_void,
                value.len() as i32,
                transmute(!0 as *const c_void));
            
            match stat {
                SQLITE_OK => Ok(()),
                _ => Err(self.error())
            }
        }
    }

    /********************************************************************
    *                                                                   *
    *                         G E T T E R S                             *
    *                                                                   *
    ********************************************************************/
    
    /// Returns integer value as i64.
    #[inline]
    fn get_i64(&self, idx: i32) -> i64 {
        unsafe { sqlite3_column_int64(self.stmt, idx) }
    }
    
    /// Returns float value as f64.
    #[inline]
    fn get_f64(&self, idx: i32) -> f64 {
        unsafe { sqlite3_column_double(self.stmt, idx) }
    }
    /// Returns text as String.
    fn get_text(&self, idx: i32) -> String {
        unsafe {
            let ptr = sqlite3_column_text(self.stmt, idx) as *const c_char;
            let data = CStr::from_ptr(ptr).to_bytes();
            String::from_utf8_lossy(data).into_owned()
        }
    }
    /// Returns blob as Vec<u8>.
    fn get_blob(&self, idx: i32) -> Vec<u8> {
        unsafe {
            let nbytes= sqlite3_column_bytes(self.stmt, idx) as usize;
            let mut data = Vec::with_capacity(nbytes);
            let ptr = sqlite3_column_blob(self.stmt, idx);
            copy(ptr, data.as_mut_ptr() as *mut c_void, nbytes);
            data
        }
    }
    
    
    /// Returns error from sqlite3.
    #[inline]
    pub fn error(&mut self) -> Error {
        Error{ code: self.err_code(), message: self.err_string(), kind: Some(Other)}
    }

    /// Get the error code from sqlite3.
    #[inline]
    fn err_code(&self) -> i32 {
        unsafe { sqlite3_errcode(self.db) }
    }

    /// Get the error message from sqlite3.
    #[inline]
    fn err_string(&self) -> String {
        unsafe { CStr::from_ptr(sqlite3_errmsg(self.db)).to_string_lossy().into_owned() }
    }
}