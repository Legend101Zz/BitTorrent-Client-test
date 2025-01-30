use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
    #[serde(rename = "emailAddress")]
    email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<String>,
}

fn serde_example() -> Result<(), Box<dyn Error>> {
    //Create Person
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        email: String::from("check@gmail.com"),
        phone: None,
    };

    //Serialize to JSON
    let json = serde_json::to_string(&person)?;
    println!("JSON: {}", json);

    // Deserialize back
    let decoded: Person = serde_json::from_str(&json)?;
    println!["Decoded: {:?}", decoded];

    Ok(())
}

fn main() {
    if let Err(e) = serde_example() {
        eprintln!("Error: {}", e);
    }
    println!("Hello, world!");
}
