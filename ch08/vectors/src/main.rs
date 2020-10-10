#![allow(unused_assignments)]
#![allow(unused_variables)]

fn main() {
    // VECTORS
    // Declaring vectors
    let v1: Vec<i32> = Vec::new(); // Must provide type if compiler cannot infer
    let v2 = vec![1, 2, 3]; // initializing with values lets compiler infer default integer type

    let mut v3 = Vec::new();
    v3.push(1);
    v3.push(2);
    v3.push(3); // Again comiler can infer integer type

    {
        let v = vec![0, 0, 0];
    } // v drops out of scope here

    // Reading elements of vectors
    let v4 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v4[2];
    println!("The third element is {}", third);

    match v4.get(5) {
        Some(elem) => println!("The sixth element is {}", elem),
        None => println!("There is no sixth element"),
    }

    // Iterating  over elements of vectors
    for i in &v4 {
        println!("{}", i)
    }

    // Use a mutable vector and mutable references to update in place
    let mut v5 = vec![1, 2, 3, 4, 5];
    for i in &mut v5 {
        *i += 50;
        println!("{}", i);
    }

    // Use a vector of enums to hold multiple types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(9.5),
        SpreadsheetCell::Text(String::from("Excellent")),
    ];
}
