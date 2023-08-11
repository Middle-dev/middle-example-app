use middle_wasm::{prelude::*, to_host};
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, JsonSchema)]
struct TestIn {
    my_str: String,
    my_num: u32,
}

#[derive(Serialize, JsonSchema)]
struct TestOut {
    my_str: String,
    my_num: u32,
}

// FIXME: Fix the macro once we feel confident in it.
// #[middle_fn]
// fn test(input: TestIn) -> TestOut {
//     println!("This is my function!");

//     TestOut { 
//         my_str: format!("I was given {}", input.my_str),
//         my_num: input.my_num + 1
//     }
// }


#[no_mangle]
pub fn simple_test() {
    println!("this was a test!");
}


#[no_mangle]
pub fn simple_test_2() -> *const u8 {
    println!("this was a slightly more complex test!");
    let output = TestOut {
        my_str: "hello world".to_owned(),
        my_num: 1
    };
    let output_json = serde_json::value::to_value(output).expect("user function output could not be serialized into JSON");
    let ptr = to_host(&output_json);
    ptr
}

#[middle_fn]
fn test(input: TestIn) -> TestOut {
    println!("This is my function!");
    TestOut {
        my_str: format!("I was given {}", input.my_str),
        my_num: input.my_num + 1,
    }
}

// #[no_mangle]
// pub fn user_fn__test(ptr: *mut u8) -> *const u8 {
//     let input_json: serde_json::Value = from_host(ptr);
//     let input: TestIn = serde_json::from_value(input_json).expect("user function input could not be serialzied");
//     let output = test(input);
//     let output_json = serde_json::value::to_value(output).expect("user function output could not be serialized into JSON");
//     let ptr = to_host(&output_json);
//     ptr
// }

// #[no_mangle]
// pub fn user_fn_info__test() -> *const u8 {
//     let fn_info = {
//         let in_schema = schemars::schema_for!(TestIn);
//         let out_schema = schemars::schema_for!(TestOut);
//         FnInfo {
//             in_schema,
//             out_schema,
//         }
//     };
//     let ptr = to_host(&fn_info);
//     ptr
// }
