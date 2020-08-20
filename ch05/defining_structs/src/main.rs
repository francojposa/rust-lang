fn main() {
    let user1 = build_user(String::from("test@test.com"), String::from("test"));

    let user2 = User {
        email: String::from("example@example.com"),
        username: String::from("example"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = XYPoint(0, 0);
}

struct User {
    // Use String vs str so the struct owns the data
    // an str is owned where the literal is declared.
    // Without lifetimes, we cannot guarantee the str
    // won't fall out of scope before the struct.
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// Tuple Structs do not have named fields
struct Color(i32, i32, i32);
struct XYPoint(i32, i32);
