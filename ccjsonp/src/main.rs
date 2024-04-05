use serde_json;
use std::env::args;
use std::fs::read_to_string;
use std::process::exit;

pub fn parse_json(contents: &str) -> bool {
    match serde_json::from_str::<serde_json::Value>(contents) {
        Ok(json) => {
            if json.is_object() && json.as_object().unwrap().is_empty() {
                true
            } else {
                false
            }
        }
        Err(_) => false,
    }
}

fn main() -> Result<(), serde_json::Error> {
    let args: Vec<String> = args().collect();

    if args.len() <= 1 {
        println!("Invalid arguments");
        exit(1);
    }

    let filename: &str = &args[2];

    let contents = match read_to_string(filename) {
        Ok(content) => content,
        Err(_) => {
            println!("Failed to read file '{}'", filename);
            exit(1);
        }
    };

    if parse_json(&contents) {
        println!("Valid JSON");
        exit(0);
    } else {
        println!("Invalid JSON");
        exit(1);
    }
}
