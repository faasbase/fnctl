use std::error::Error;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Value};

// TODO: replace the fields in this struct with your own input data structure
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct Input {
	pub firstname: String,
	pub lastname: String,
}

// TODO: replace the fields in this struct with your own output data structure
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct Output {
	pub name: String,
	pub data: Value,
}

pub fn handler(data: Input) -> Result<Output, Box<dyn Error>> {
	// TODO: implement your function here

	let output = Output {
		name: "hello".to_string(),
		data: serde_json::to_value(data)?,
	};
	Ok(output)
}
