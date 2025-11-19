fn classify_number(n: i32) -> &'static str {
    if n < 0 {
        "negative"
    } else if n == 0 {
        "zero"
    } else {
        "positive"
    }
}

fn main() {
    println!("Hello, world!");

    println!("5 is {}", classify_number(5));
    println!("-3 is {}", classify_number(-3));
    println!("0 is {}", classify_number(0));
    println!("10 is {}", classify_number(10)
);
}
