// Entry point for the real Lambda POC.
// For now we run the handler once locally with a fake event.
// Later, this main can be switched to the full Lambda runtime loop.

use lambda_runtime::{Context, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

// Simple event payload we'll accept from Lambda (or from a local test).
#[derive(Debug, Deserialize)]
struct Request {
	path: String,
	id: Option<u64>,
}

// Simple response we'll return.
#[derive(Debug, Serialize)]
struct Response {
	message: String,
}

// Core handler logic. In real Lambda this will be called for each invocation.
async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
	let (event, _context) = event.into_parts();

	let id_info = match event.id {
		Some(id) => format!(" for id={id}"),
		None => String::from(""),
	};

	let message = format!("Hello from Rust Lambda on path '{}'{}", event.path, id_info);
	Ok(Response { message })
}

// Local entry point: construct a fake event and call the handler once.
#[tokio::main]
async fn main() -> Result<(), Error> {
    let event = Request {
        path: "/local/test".to_string(),
        id: Some(123),
    };

    let ctx = Context::default();
    let lambda_event = LambdaEvent::new(event, ctx);

    let resp = function_handler(lambda_event).await?;
    println!("Local handler response: {}", resp.message);

    Ok(())
}
