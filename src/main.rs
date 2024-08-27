use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result};

mod card;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short)]
    db: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    List {
        #[arg(short)]
        card: Option<String>, //If given it will try to parse the file
    },
    Info {
        #[arg(short)]
        card: String,
    },
    Move {
        #[arg(short)]
        card: String, // If this is a file, then it will first try to parse that file
        #[arg(short)]
        list: String,
    },
    New {
        #[arg(short)]
        list: String,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();
    let conn = Connection::open(args.db)?;

    match args.command {
        Commands::List { card } => {
            for list in card::List::all(&conn) {
                println!("{} with ID {}", list.name, list.id);
            }
        }
        _ => todo!(),
    }

    Ok(())
}
