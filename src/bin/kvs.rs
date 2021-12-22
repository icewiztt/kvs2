use clap::{AppSettings, Parser, Subcommand};
use kvs::{KvStore, TError, Result};
use std::env::current_dir;
use std::process::exit;

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
#[clap(setting(AppSettings::SubcommandRequiredElseHelp))]
struct Cli {
    #[clap(subcommand)]
    action: Actions,
}

#[derive(Subcommand)]
enum Actions {
    Set { key: String, value: String },
    Get { key: String },
    Rm { key: String },
}

fn main() -> Result<()>{
    let cli = Cli::parse();

    match &cli.action {
        Actions::Set {key, value} => {
            let mut store = KvStore::open(current_dir()?)?;
            store.set(key.to_string() , value.to_string())?;
        }
        Actions::Get { key } => {
            let mut store = KvStore::open(current_dir()?)?;
            if let Some(value) = store.get(key.to_string())? {
                println!("{}", value);
            }else{
                println!("Key not found");
            }
            
        }
        Actions::Rm { key } => {
            let mut store = KvStore::open(current_dir()?)?;
            match store.remove(key.to_string()) {
                Ok(()) => (),
                Err(TError::NonExistentKey) => {
                    println!("Key not found");
                    exit(1);
                }
                Err(e) => return Err(e)
            }
        }
    }
    Ok(())
}