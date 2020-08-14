fn main() {
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);

    change(&mut s1);
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);

    let mut s = String::from("hello");
    let r1 = &s; // No problem
    let r2 = &s; // No problem, the references are immutable
    println!("{} and {}", r1, r2); // r1 & r2 not used after this

    let mut r3 = &s; // Mutable reference, but r1 and r2 are already out of scope
    println!("{}", r3);
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn change(s: &mut String) {
    s.push_str(", world");
}

fn dangle() -> &String {
    // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String
    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!

fn no_dangle() -> String {
    let s = String::from("hello");
    s // return the string directly, ownership is transferred out
}
