use middle_wasm::{prelude::*, to_host};
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, JsonSchema)]
struct TestIn {
    /// Enter your name
    name: String,
}

#[derive(Serialize, JsonSchema)]
struct TestOut {
    /// A hello message is returned
    hello_message: String,
}

#[middle_fn("This function returns `Hello, {input.name}`")]
fn test(input: TestIn) -> TestOut {
    TestOut { 
        hello_message: format!("Hello, {}!", input.name),
    }
}