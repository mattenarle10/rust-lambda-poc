// Entry point for the real Lambda POC.
// For now this is just a placeholder main; we'll replace
// it with a proper lambda_runtime handler as the next step.

use lambda_runtime::{service_fn, Error, LambdaEvent};
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

// Lambda entry point. Locally, `cargo run` will start the runtime
// and immediately run our handler whenever Lambda invokes it.
#[tokio::main]
async fn main() -> Result<(), Error> {
	lambda_runtime::run(service_fn(function_handler)).await
}
