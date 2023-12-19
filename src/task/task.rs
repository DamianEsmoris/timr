use piechart::Color;
use std::io::{self, Write};
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub name: String,
    pub color: String,
}

impl Task {
    pub fn new(name: &str) -> Task {
        let mut user_input = String::new();
        print!("Type a hex color for the new task: ");
        std::io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let t: Task = Task {
            name: name.to_string(),
            color: (&user_input[0..6]).to_string()
        };

        t
    }
}

pub fn hex_to_rgb_piechart(str: &String) -> piechart::Color {
    let r = u8::from_str_radix(&str[0..2], 16).unwrap();
    let g = u8::from_str_radix(&str[2..4], 16).unwrap();
    let b = u8::from_str_radix(&str[4..6], 16).unwrap();
    Color::RGB(r, g, b)
}


pub fn hex_to_rgb(str: &String) -> (u8, u8, u8) {
    let r = u8::from_str_radix(&str[0..2], 16).unwrap();
    let g = u8::from_str_radix(&str[2..4], 16).unwrap();
    let b = u8::from_str_radix(&str[4..6], 16).unwrap();
    (r, g, b)
}



