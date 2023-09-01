use middle_wasm::prelude::*;

#[middle_fn("This function returns `Hello, {input.name}`")]
fn make_hello(name: String) -> String {
    format!("Hello, {name}!")
}