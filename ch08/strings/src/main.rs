#![allow(unused_assignments)]
#![allow(unused_variables)]

fn main() {
    // Declaring strings
    let mut s = String::new();
    let data = "initial contents";
    s = data.to_string();
    s = "initial contents".to_string();
    s = String::from("initial contents");

    s = String::from("foo");
    s.push_str("bar");

    s = String::from("lo");
    s.push('l');

    // Concatenating strings
    let s1 = String::from("Hello");
    let s2 = String::from(", World");
    let s3 = s1 + &s2; // s1 gets moved here and can no longer be used

    let s4 = String::from("tic");
    let s5 = String::from("tac");
    let s6 = String::from("toe");
    let s7 = format!("{}-{}-{}", s4, s5, s6); // format macro does not take ownership

    // Indexing into Strings
    // Rust allows UTF-8 strings, where a single "letter" may be more than one
    // byte, but the underlying data structure is still `Vec<u8>`
    // So indexing like a vector could cause you to pull out a byte that doesn't even
    // represent a full character and is meaning less on its own.

    // Rust does NOT allow you to grab single indices from a String:
    let hello = String::from("Здравствуйте");
    // let a = &hello[0]  // Does not compile

    // Rust will allow you to slice strings with byte indexes
    // but will panic at runtime if the byte index slices a character in half
    let s = &hello[0..4]; // Good, gets first two of the two-byte characters

    // let s = &hello[0..3]; // panic
    // thread 'main' panicked at 'byte index 3 is not a char boundary;
    // it is inside 'д' (bytes 2..4) of `Здравствуйте`'

    // Iterating over strings
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
