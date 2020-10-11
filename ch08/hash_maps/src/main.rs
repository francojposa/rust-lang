#![allow(unused_assignments)]
#![allow(unused_variables)]

use std::collections::HashMap;

fn main() {
    // Hash maps are homogeneous - must have all same type of key and val
    //  Hash maps are allocated on the heap due to their dynamic nature

    // Hash Maps Declaration
    // Basic empty declaration and insert
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 35);
    // Zipping up two iterables into a hashmap with collect()
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let scores = vec![10, 35];
    // Need HashMap annotation because iterables can collect() into many diff types
    // However, we can use _, _ and let Rust infer the types from the vectors
    let mut scores: HashMap<_, _> = teams.into_iter().zip(scores.into_iter()).collect();

    // Hash Map Ownership
    // Types implementing `Copy` are copied into a hash mp
    // Owned types like String get moved into the hash map
    let key = String::from("Favorite Color");
    let val = String::from("Grey");
    let mut map = HashMap::new();
    map.insert(key, val);
    // key and val are invalid at this point
    // println!("{}", key); // See compiler errors

    // Hash Map Access
    // `get` returns type `Option(&V)`, so the actual return type
    // is either `None` or the value wrapped in `Some`
    let team_name = String::from("Blue");
    let team_score = scores.get(&team_name);
    match team_score {
        Some(val) => println!("{} team score is {}", team_name, val),
        None => println!("{} team score not found", team_name),
    }
    // Iteration does not guarantee any sort of orer
    for (key, val) in &scores {
        println!("{}: {}", key, val);
    }

    // Hash Map Updates
    // Insert only if key has no value
    // Hash Maps have `entry` method that returns an `Entry` enum type
    // wrapping `Occupied` or `Vacant` types
    let red_score = scores.entry(String::from("Red")); // Vacant
    scores.entry(String::from("Blue")).or_insert(20); // Occupied, no effect
    println!("{:?}", scores); // ? Uses `Debug` implementation vs normal `Display`
}
