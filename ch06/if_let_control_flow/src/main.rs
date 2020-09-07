fn main() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // Equivalent to above
    // If-let is more concise than match but does not
    // have the safety of forced exhaustiveness
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        // Same as _ in match
        println!("anything else");
    }
}
