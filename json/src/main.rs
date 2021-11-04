use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
}

fn main() {
    let data = fs::read_to_string("./data.json").expect("Unable to read file");
    let person: Person = serde_json::from_str(&data).expect("JSON was not well-formatted");
    println!("{}", person.name);
}
