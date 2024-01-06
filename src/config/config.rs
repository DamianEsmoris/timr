use std::{fs::File, io::ErrorKind};
use dotenv::dotenv;
use crate::Query;

#[derive(Clone)]
pub struct Config<'a>{
    pub server_ip: String,
    pub server_port: u16,
    pub db_path: String,
    pub query: &'a Query<'a>,
}

impl<'a> Config<'a>{
    pub fn new(query: &'a Query, validate: bool) -> Config<'a>{
        dotenv().ok();
        let server_ip: String = std::env::var("SERVER_IP")
            .expect("The SERVER_IP must be set.");
        let server_port: u16 = std::env::var("SERVER_PORT")
            .expect("The SERVER_PORT must be set.")
            .parse::<u16>().unwrap();
        let mut db_path: String = std::env::var("DB_PATH")
            .expect("The DB_PATH must be set!!");

        db_path = db_path.replace("EXEC_PATH", query.filepath); 
        let config = Config {
            server_ip,
            server_port,
            db_path,
            query
        };

        match validate {
            true => {
                match config.validate() {
                    true => config,
                    false => panic!(),
                }
            }
            false => config,
        }
    }

    fn validate(&self) -> bool {
       File::open(&self.db_path)
            .expect("Cannot open the db file!");

        let result = File::open(
            format!("{}/static/index.html", self.query.filepath));

        match result {
            Err(err) => {
                match err.kind() {
                    ErrorKind::NotFound => {
                        println!("static/ not found in the filepath\n the webview is not going to work properly");
                        false
                    }
                    _ => {
                        println!("{}", err);
                        false
                    }
                }
            }
            Ok(_) => true
        }


    }


}

