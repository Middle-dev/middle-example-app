use middle_wasm::*;
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
struct TestIn {
    my_str: String,
    my_num: u32,
}

#[derive(Serialize)]
struct TestOut {
    my_str: String,
    my_num: u32,
}

#[middle_fn]
fn test(input: TestIn) -> TestOut {
    println!("This is my function!");

    TestOut { 
        my_str: format!("I was given {}", input.my_str),
        my_num: input.my_num + 1
    }
}
