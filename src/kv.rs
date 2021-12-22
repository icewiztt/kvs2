use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::BufWriter;
use std::collections::HashMap;
use crate::{TError, Result};

#[derive(Debug, Serialize, Deserialize)]
enum Actions {
    Set { key: String, value: String },
    Get { key: String },
    Rm { key: String },
}

impl Actions{
    fn set(key: String, value:String) -> Actions {
        Actions::Set{key, value}
    }
    fn get(key: String) -> Actions {
        Actions::Get{key}
    }
    fn rm(key: String) -> Actions {
        Actions::Rm{key}
    }
}

#[derive(Default)]
pub struct KvStore {
    path : PathBuf,
    gen_id : i32,
    store : HashMap<String,String>
}

impl KvStore {
    //bincode crate was choosen for its performance and maturity
    //also I want to try something other than serde_json
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let message = Actions::set(key, value);
        // let file = File::create("foo.txt").unwrap();
        // let mut writer = BufWriter::new(&file);
        
        // bincode::serialize_into(&mut writer, &message)?;
        if let Actions::Set {key, value} = message {
            self.store.insert(key,value);
        }
        Ok(())
    }
    pub fn get(&mut self, _key: String) -> Result<Option<String>> {
        panic!();
    }
    pub fn remove(&mut self, key: String) -> Result<()> {
        if self.store.contains_key(&key){
            let message = Actions::rm(key);
            // let file = File::create("foo.txt").unwrap();
            // let mut writer = BufWriter::new(&file);
            // bincode::serialize_into(&mut writer, &message)?;
            if let Actions::Rm {key} = message {
                self.store.remove(&key).expect("key not found");
            }
            Ok(())
        }else {
            Err(TError::NonExistentKey)
        }
        
    }
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        Ok(KvStore{
            path : path.into(),
            gen_id : 0,
            store : HashMap::new()
        })
    }
}
