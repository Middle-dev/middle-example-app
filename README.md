# middle-example-app
An example for how to create an app in Middle

## Building
Once you're ready to test your app on Middle, compile it with the following command:

It's important that target-feature multivalue be on.
All of Middle's WASM execution code depends on it, and it makes executing your WASM a bit more efficient.
Also, for reasons, you must compile your app with the --release flag turned on.

    RUSTFLAGS="-C target-feature=+multivalue" cargo wasi build --release
