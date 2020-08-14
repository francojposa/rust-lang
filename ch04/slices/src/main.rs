fn main() {
    let string = String::from("hello world");

    // first_word works onslices of `String`
    let word = first_word(&string[..]);

    let string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&string_literal[..]);

    // string_literal is already a str so we can pass directly
    let word = first_word(string_literal);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
