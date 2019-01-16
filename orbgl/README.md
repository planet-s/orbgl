# OrbGL - The Rust Graphics Library

The Orbital Graphics Library is a library for rendering 2D and 3D graphics.

## Running the examples

You find the examples in the `examples/` directory.

You can start the drawing example by executing the following command:

```text
cargo run --example simple
```

## Running the web examples

> The WebRenderEngine is only available for wasm32 and asmjs targets

1. Install [cargo-web]:

       $ cargo install -f cargo-web

3. Go into `examples/web/simple` and start the example using one of these commands:

    * Compile to [WebAssembly] using Rust's native WebAssembly backend:

          $ cargo web start --target=wasm32-unknown-unknown

    * Compile to [asm.js] using Emscripten:

          $ cargo web start --target=asmjs-unknown-emscripten

    * Compile to [WebAssembly] using Emscripten:

          $ cargo web start --target=wasm32-unknown-emscripten

4. Visit `http://localhost:8000` with your browser.

For the `*-emscripten` targets `cargo-web` is not necessary, however
the native `wasm32-unknown-unknown` which doesn't need Emscripten
**requires** `cargo-web` to work!