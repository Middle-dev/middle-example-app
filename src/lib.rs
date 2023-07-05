use middle_wasm::*;

#[middle_fn]
fn test(input: WasmMainCall) -> WasmMainResult {
    println!("hello world");

    WasmMainResult::Ok
}