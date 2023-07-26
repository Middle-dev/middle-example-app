use middle_wasm::*;

#[middle_fn]
fn test(input: WasmMainCall) -> WasmMainResult {
    println!("This is my workflow!");

    WasmMainResult::Ok
}