// Collections Types
// Hash Maps 
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{team_name}: {score}");

    // If you want to print all key-value pairs:
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
