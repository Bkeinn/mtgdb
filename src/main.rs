use card::{Card, CardQuery, List};
use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result};
use std::io;

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
        Commands::Info { card } => {
            println!("{} in:", &card);
            let vec = vec![card];
            let query = &card::CardQuery::names(vec, &conn)[0];
            for card in &query.cards {
                println!("\t{} is deck = {}", card.list.name, card.list.deck);
            }
        }
        Commands::Move { card, list } => {
            let vec = vec![card];
            let cardquery = &card::CardQuery::names(vec, &conn)[0];
            let list = List::by_name(list, &conn);
            let mut selected_card = None;
            for card in &cardquery.cards {
                if !card.list.deck {
                    selected_card = Some(card);
                    break;
                }
            }
            let selected_card = match selected_card {
                Some(card) => card,
                None => {
                    println!("All versions of this card are allready in decks, whitch one would you like to move");
                    let mut card = None;
                    loop {
                        for (num, card) in cardquery.cards.iter().enumerate() {
                            println!("{}\t{}", num, card.list.name);
                        }
                        let mut input = String::new();
                        if io::stdin().read_line(&mut input).is_err() {
                            println!("This isn't a valid input");
                            continue;
                        }
                        let number: usize = match input.trim().parse() {
                            Ok(num) => num,
                            Err(_) => {
                                println!("This isn't a valid input");
                                continue;
                            }
                        };
                        card = Some(match cardquery.cards.get(number) {
                            Some(card) => card,
                            None => {
                                println!("This isn't a valid input");
                                continue;
                            }
                        });
                        break;
                    }
                    card.unwrap()
                }
            };
            Card::move_to_list(selected_card, &list, &conn)
                .expect("Card could not be moved to list");
        }
        _ => todo!(),
    }

    Ok(())
}
