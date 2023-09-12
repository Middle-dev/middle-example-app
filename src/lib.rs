use std::time::Duration;

use middle_wasm::prelude::*;


#[middle_fn]
/// This function returns `Hello, {input.name}`
fn make_hello(name: String) -> String {
    format!("Hello, {name}!")
}

#[middle_workflow]
/// This is a test workflow, which can pause, and stop and resume later.
fn test_workflow(to_print: String) -> Resumable<String> {
    // This function prints something to Middle's console.
    mprint("making a request...");

    // This code will repeatedly execute until a non-500 error is returned.
    // There's an upper-bound to the number of steps executed, after which an error will result.
    let out = loop {
        let out = middle_wasm::RequestBuilder::get("https://middle.app")
            .call()
            .unwrap();

        if out.code() >= 500 {
            // Waiting 5 seconds for 5xx error to resolve
            middle_wasm::pause(Duration::from_secs(5))?;
        } else {
            break out;
        }
    };
    
    // Code from this point on will only be run once the pause duration has passed.
    mprint(format!("we were supposed to print... {to_print}"));
    mprint(format!("here's the result of our call... {}", out.code()));
    
    // We have to return Ready when the workflow is done.
    Resumable::Ready("I'm done!".to_string())
}