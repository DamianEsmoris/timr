use std::env;
use rusqlite::{Connection, Result};
use crate::{Config, Query};

pub struct Db {
    pub conn: Connection,
}

impl Db {
    pub fn new() -> Result<Self> {
        let args: Vec<String> = env::args().collect();
        let query: Query = Query::new(&args);
        let config: Config = Config::new(&query, false);
        let conn = Connection::open(config.db_path)
            .expect("Error connecting to the database!");
        Ok(Db { conn })
    }
}
