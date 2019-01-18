# OrbGL Web - The Orbital Graphics Library for the web

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./../LICENSE)

This is the web version of OrbGL and is compatible with stdweb.

## Minimal Example

```rust
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{
    document,
    window,
    CanvasRenderingContext2d
};

use stdweb::web::html_element::CanvasElement;

use orbgl_web::prelude::*;

fn main() {
    stdweb::initialize();

    let w = 800;
    let h = 600;

    let canvas: CanvasElement = document().query_selector( "#canvas" ).unwrap().unwrap().try_into().unwrap();
    canvas.set_width(w);
    canvas.set_height(h);
   
    let surface = WebSurface::new(w, h, canvas.get_context().unwrap());
    let render_engine = WebRenderEngine::new(surface);
    let mut canvas = Canvas::new(render_engine.clone());
    canvas.set_fill_style(Color::rgba(0, 0, 0, 255));
    canvas.fill_rect(10.0, 10.0, 100.0, 75.0);

    stdweb::event_loop();
}
```

## Running the additional examples

1. Install [cargo-web]:

       $ cargo install -f cargo-web

3. Go into `examples/simple` and start the example using one of these commands:

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

## Dependencies

* [stdweb](https://github.com/koute/stdweb): drawing on a web canvas
* [orbgl_api](https://gitlab.redox-os.org/redox-os/orbgl/orbgl_api): base orbgl api