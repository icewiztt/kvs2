use std::path::PathBuf;
use std::fmt;

pub type Result<T> = std::result::Result<T, MyErrorType>;

#[derive(Debug, Clone)]
pub struct MyErrorType;

impl fmt::Display for MyErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error")
    }
}

#[derive(Default)]
pub struct KvStore {

}

impl KvStore {
    pub fn set(&mut self, key: String, value: String) -> Result<()>{
        panic!();
    }
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        panic!();
    }
    pub fn remove(&mut self, key: String) -> Result<()>{
        panic!();
    }
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore>{
        panic!();
    }
}