use std::time::Instant; // small timer helper

fn classify_number(n: i32) -> &'static str {
    // simple function returning a label
    if n < 0 {
        "negative"
    } else if n == 0 {
        "zero"
    } else {
        "positive"
    }
}

fn shout(message: String) -> String {
    // takes ownership of the String and returns a new one
    format!("{}!!!", message.to_uppercase())
}

fn print_message(message: &str) {
    // only borrows a string slice, does not take ownership
    println!("msg: {message}");
}

fn benchmark_sum(label: &str, limit: u64) {
    // measure how long a simple loop takes
    let start = Instant::now();

    let mut sum: u64 = 0; // sum owns this counter
    for i in 0..limit {
        sum += i;
    }

    let elapsed = start.elapsed();
    println!(
        "{label}: sum={sum} for 0..{limit} took {:?} (~{:.6} s)",
        elapsed,
        elapsed.as_secs_f64()
    );
}

fn main() {
    // program entry point
    println!("Hello, world!");

    // basic control-flow example
    println!("5 is {}", classify_number(5));
    println!("-3 is {}", classify_number(-3));
    println!("0 is {}", classify_number(0));
    println!("10 is {}", classify_number(10));

    // ownership: msg is moved into shout
    let msg = String::from("hello rust");
    let loud = shout(msg);

    // borrowing: we pass a reference to loud
    print_message(&loud);
    println!("loud = {loud}");

    // tiny benchmark to feel speed (try: cargo run --release)
    benchmark_sum("sum up to 20 million", 20_000_000);
}
