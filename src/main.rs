// Entry point for the real Lambda POC.
// This uses lambda_runtime to run our handler for each Lambda invocation.

use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

// Simple event payload we'll accept from Lambda (or from a local test).
#[derive(Debug, Deserialize)]
struct Request {
	path: String,
	id: Option<u64>,
	command: Option<String>,
}

// Simple response we'll return.
#[derive(Debug, Serialize)]
struct Response {
	message: String,
}

// Core handler logic. In real Lambda this will be called for each invocation.
async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
	let (event, _context) = event.into_parts();

	let command = event.command.as_deref().unwrap_or("hello");

	let message = match command {
		"hello" => {
			let id_info = match event.id {
				Some(id) => format!(" for id={id}"),
				None => String::from(""),
			};
			format!("Hello from Rust Lambda on path '{}'{}", event.path, id_info)
		}
		"health" => "ok".to_string(),
		other => format!("unknown command: {other}"),
	};

	Ok(Response { message })
}

// Real Lambda entry point: the runtime will call our handler
// with real events from AWS.
#[tokio::main]
async fn main() -> Result<(), Error> {
	run(service_fn(function_handler)).await
}
