use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::BufWriter;
pub use error::Result;
use error::TError;
mod error;

#[derive(Debug, Serialize, Deserialize)]
enum Actions {
    Set { key: String, value: String },
    Get { key: String },
    Rm { key: String },
}

#[derive(Default)]
pub struct KvStore {

}

impl KvStore {
    //bincode crate was choosen for its performance and maturity
    //also I want to try something other than serde_json
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let message = Actions::Set{key:key, value:value};
        let file = File::open("foo.txt").unwrap();
        let writer = BufWriter::new(&file);

        bincode::serialize_into(writer, &message)?;
        panic!();   
    }
    pub fn get(&mut self, _key: String) -> Result<Option<String>> {
        panic!();
    }
    pub fn remove(&mut self, key: String) -> Result<()> {
        Ok(())
    }
    pub fn open(_path: impl Into<PathBuf>) -> Result<KvStore> {
        panic!();
    }
}
