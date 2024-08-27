use anyhow::{Context, Result};
use rusqlite::Connection;

pub struct CardQuery {
    cards: Vec<Cards>,
    data_card: Data_cards,
    name: Data_name,
}

pub struct Cards {
    card: u64,
    id: u64,
}

pub struct Data_cards {
    name: u64,
    id: u64,
}

pub struct Data_name {
    name: String,
    id: u64,
}

pub struct List {
    pub name: String,
    pub id: u64,
}

impl List {
    pub fn all(conn: &Connection) -> Vec<List> {
        let mut query = conn
            .prepare("SELECT _id, name FROM 'lists'")
            .context("Query not possible")
            .unwrap();
        let list_iter: Result<Vec<List>, rusqlite::Error> = query
            .query_map([], |row| {
                Ok(List {
                    name: row.get(1).context("No name").unwrap(),
                    id: row.get(0).context("No id").unwrap(),
                })
            })
            .unwrap()
            .collect();

        let list_vec = list_iter.unwrap();
        return list_vec;
    }
}
