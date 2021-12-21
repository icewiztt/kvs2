use clap::{AppSettings, Parser, Subcommand};

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

fn main() {
    let cli = Cli::parse();

    std::process::exit(match &cli.action {
        Actions::Set { .. } => {
            eprintln!("unimplemented");
            1
        }
        Actions::Get { .. } => {
            eprintln!("unimplemented");
            1
        }
        Actions::Rm { .. } => {
            eprintln!("unimplemented");
            1
        }
    });
}