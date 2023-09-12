# middle-example-app
An example for how to create an app in Middle

## Building
Once you're ready to test your app on Middle, compile it with the following command:

    cargo build --target wasm32-unknown-unknown

The above will compile a Web Assembly binary that's rather large, as it has all the debug symbols turned on. This is great for debugging.

However, if you want a smaller Web Assembly binary, you can build with the following command.

    cargo build --target wasm32-unknown-unknown --release
