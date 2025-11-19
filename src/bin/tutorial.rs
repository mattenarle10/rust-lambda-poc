// use std::time::Instant; // small timer helper

// #[allow(dead_code)]
// fn classify_number(n: i32) -> &'static str {
//     // simple function returning a label
//     if n < 0 {
//         "negative"
//     } else if n == 0 {
//         "zero"
//     } else {
//         "positive"
//     }
// }

// #[allow(dead_code)]
// fn shout(message: String) -> String {
//     // takes ownership of the String and returns a new one
//     format!("{}!!!", message.to_uppercase())
// }

// #[allow(dead_code)]
// fn print_message(message: &str) {
//     // only borrows a string slice, does not take ownership
//     println!("msg: {message}");
// }

// #[allow(dead_code)]
// fn benchmark_sum(label: &str, limit: u64) {
//     // measure how long a simple loop takes
//     let start = Instant::now();

//     let mut sum: u64 = 0; // sum owns this counter
//     for i in 0..limit {
//         sum += i;
//     }

//     let elapsed = start.elapsed();
//     println!(
//         "{label}: sum={sum} for 0..{limit} took {:?} (~{:.6} s)",
//         elapsed,
//         elapsed.as_secs_f64()
//     );
// }

// simple data structure similar to a tiny HTTP/Lambda request
#[derive(Debug)]
struct Request {
    path: String,
    id: Option<u64>,
}

// enum with a couple of possible actions
#[derive(Debug)]
enum Action {
    ListBuckets,
    GetItem(u64),
}

// handler that returns Result to represent success or error
fn handle_action(action: Action) -> Result<String, String> {
    match action {
        Action::ListBuckets => Ok("listing buckets...".to_string()),
        Action::GetItem(id) => {
            if id == 0 {
                Err("id must be > 0".to_string())
            } else {
                Ok(format!("getting item with id={id}"))
            }
        }
    }
}

fn main() {
    // program entry point
    println!("Hello, world from tutorial!");

    // --- Example 1: basic control-flow (classify_number) ---
    // println!("5 is {}", classify_number(5));
    // println!("-3 is {}", classify_number(-3));
    // println!("0 is {}", classify_number(0));
    // println!("10 is {}", classify_number(10));

    // --- Example 2: ownership + borrowing + clone ---
    // let msg = String::from("hello rust");
    // let loud = shout(msg.clone());
    // println!("msg again = {msg}");
    // print_message(&loud);
    // println!("loud = {loud}");

    // --- Example 3: tiny benchmark (uncomment to run) ---
    // benchmark_sum("sum up to 20 million", 20_000_000);

    // --- Example 4: structs + enums + Result (Lambda-ish handler) ---
    let req = Request {
        path: "/items".to_string(),
        id: None,
    };

    println!("request path = {}", req.path);

    let action = match req.id {
        Some(id) => Action::GetItem(id),
        None => Action::ListBuckets,
    };

    match handle_action(action) {
        Ok(msg) => println!("handler success: {msg}"),
        Err(err) => println!("handler error: {err}"),
    }
}
