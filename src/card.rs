use core::panic;

use anyhow::{Context, Result};
use rusqlite::{params, Connection};

#[derive(Debug)]
pub struct CardQuery {
    pub cards: Vec<Card>,
    pub data_card: Vec<Data_cards>,
    pub data_name: Data_name,
}

#[derive(Debug)]
pub struct Card {
    pub card: u64,
    pub id: u64,
    pub list: List,
}

#[derive(Debug)]
pub struct Data_cards {
    name: u64,
    id: u64,
}

#[derive(Debug)]
pub struct Data_name {
    name: String,
    id: u64,
}

#[derive(Debug)]
pub struct List {
    pub name: String,
    pub id: u64,
    pub deck: bool,
}

impl CardQuery {
    pub fn names(names: Vec<String>, conn: &Connection) -> Vec<CardQuery> {
        let data_names = Data_name::names_to_ids(names, conn);
        let data_cards = Data_cards::ids_to_absolute(
            data_names
                .iter()
                .map(|Data_name { name, id }| *id)
                .collect(),
            conn,
        );
        let cards = Card::by_id(
            data_cards
                .iter()
                .map(|vec| vec.iter().map(|Data_cards { name, id }| *id).collect())
                .collect(),
            conn,
        );
        cards
            .into_iter()
            .zip(data_cards)
            .zip(data_names)
            .map(|((cards, data_card), data_name)| CardQuery {
                cards,
                data_card,
                data_name,
            })
            .collect()
    }
}

impl Card {
    pub fn by_id(ids: Vec<Vec<u64>>, conn: &Connection) -> Vec<Vec<Card>> {
        let mut result = Vec::new();
        let mut query = conn
            .prepare("SELECT _id, list FROM 'cards' Where card = (?1)")
            .unwrap();
        for card in ids {
            let mut all_options = Vec::new();
            let result = &mut result;
            for id in card {
                let mut rows = query.query(params![id]).unwrap();
                while let Some(row) = rows.next().unwrap() {
                    all_options.push(Card {
                        card: id,
                        id: row.get(0).unwrap(),
                        list: List::by_id(row.get(1).unwrap(), conn),
                    });
                }
            }
            result.push(all_options);
        }
        println!("Cards: {:#?}", &result);
        return result;
    }
}

impl Data_cards {
    pub fn ids_to_absolute(ids: Vec<u64>, conn: &Connection) -> Vec<Vec<Data_cards>> {
        let mut result = Vec::new();
        let mut query = conn
            .prepare("SELECT _id FROM 'data_cards' WHERE name = (?1)")
            .unwrap();
        for id in ids {
            let result = &mut result;
            let mut rows = query.query(params![id]).unwrap();
            let mut all_options = Vec::new();
            while let Some(row) = rows.next().context("No query result").unwrap() {
                all_options.push(Data_cards {
                    name: id,
                    id: row.get(0).unwrap(),
                })
            }
            result.push(all_options);
        }
        println!("Data_cards: {:#?}", &result);
        return result;
    }
}

impl Data_name {
    pub fn names_to_ids(names: Vec<String>, conn: &Connection) -> Vec<Data_name> {
        let mut result = Vec::new();
        let mut query = conn
            .prepare("SELECT _id FROM 'data_names' WHERE name = (?1)")
            .unwrap();
        for name in names {
            let result = &mut result;
            let mut rows = query.query(params![name]).unwrap();
            if let Some(row) = rows.next().context("No query result").unwrap() {
                result.push(Data_name {
                    name: name,
                    id: row.get(0).unwrap(),
                });
            }
        }
        println!("Data_names: {:#?}", &result);
        return result;
    }
}

impl List {
    fn new(name: String, id: u64) -> Self {
        let deck = deck(&name);
        List { deck, name, id }
    }
    fn by_id(id: u64, conn: &Connection) -> List {
        let mut query = conn
            .prepare("SELECT name FROM 'lists' Where _id = (?1)")
            .unwrap();
        if let Some(row) = query.query(params![id]).unwrap().next().unwrap() {
            return List::new(row.get(0).unwrap(), id);
        }
        panic!("ERROR did not List missing");
    }

    pub fn all(conn: &Connection) -> Vec<List> {
        let mut query = conn
            .prepare("SELECT _id, name FROM 'lists'")
            .context("Query not possible")
            .unwrap();
        let list_iter: Result<Vec<List>, rusqlite::Error> = query
            .query_map([], |row| {
                Ok(List::new(
                    row.get(1).context("No name").unwrap(),
                    row.get(0).context("No id").unwrap(),
                ))
            })
            .unwrap()
            .collect();

        let list_vec = list_iter.unwrap();
        return list_vec;
    }
}

fn deck(s: &str) -> bool {
    s.starts_with("Deck")
}
