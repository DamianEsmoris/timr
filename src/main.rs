use timr::{Query, Config};
use std::env;
    
fn main() {
    let args: Vec<String> = env::args().collect();
    let query: Query = Query::new(&args);
    let config: Config = Config::new(&query, true);
    
    let _ = query.process_args();
}
