fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    // Shadowing
    let y = 10;
    let y = y + 1;
    let y = y + 1;
    println!("The value of y is {}", y);

    // Scalar types
    let _a = 10; // i32
    let _b: i64 = 10; // i64

    let a = 5.0; // f64
    let b: f32 = 3.0; // f32

    // Basic scalar operations
    let sum = a + b;
    println!("Sum of {} and {} is {}", a, b, sum);

    let difference = a - b;
    println!("Difference of {} and {} is {}", a, b, difference);

    let product = a * b;
    println!("Product of {} and {} is {}", a, b, product);

    let quotient = a / b;
    println!("Quotient of {} and {} is {}", a, b, quotient);

    let remainder = a - b;
    println!("Remainder of {} and {} is {}", a, b, remainder);

    // Compound type: Tuple
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("The members of the tuple are: {}, {}, {}", x, y, z);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!(
        "The members of the tuple are: {}, {}, {}",
        five_hundred, six_point_four, one
    );

    // Compound type: Array
    // Array must be all one type
    // Stack allocated (fixed length)
    let _arr = [1, 2, 3, 4, 5];
    let _arr: [i32; 5] = [1, 2, 3, 4, 5];
    let _arr_3 = [3; 5]; // [3, 3, 3, 3, 3]
}
