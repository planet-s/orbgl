# OrbGL Shapes - Shapes extension of the Orbital Graphics Library (OrbGL)

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./../LICENSE)

This is an optinal extion for the Orbital Graphics Library (OrbGL). It gives you the ability to use and draw shapes like Rectangle and ImageElement instead of calling plain drawing functions. It is compatible with orbgl and orgl_web.

## Minimal Example

```rust
use orbclient::{EventOption, Renderer, Window};
use orbgl::prelude::*;
use orbgl_shapes::prelude::*;

fn main() {
    let w = 800;
    let h = 600;
    let (width, height) = orbclient::get_display_size().unwrap();
    let mut window = Window::new_flags(
        (width as i32) / 4,
        (height as i32) / 4,
        w,
        h,
        "OrbGL",
        &[orbclient::WindowFlag::Async],
    )
    .unwrap();
    let (win_w, win_h) = (w, h);
    window.rect(0, 0, win_w, win_h, Color::rgb(255, 255, 255));

    let surface = FramebufferSurface::new(800, 600, window.data_mut().as_mut_ptr() as *mut u8);
    let render_engine = OrbGLRenderEngine::new(surface.clone());
    let mut canvas = Canvas::new(render_engine.clone());

    let mut rectangle = Rectangle::create()
        .with_rect(10.0, 10.0, 100.0, 75.0)
        .with_background(Brush::from("#000000"))
        .build();

    canvas.render_shape(&mut rectangle);

    window.sync();

    'event: loop {
        for orbital_event in window.events() {
            match orbital_event.to_option() {
                EventOption::Quit(_quit_event) => break 'event,
                _ => (),
            };
        }
    }
}
```

## Additional Examples

You find the examples in the `examples/` directory.

You can start the widgets example by executing the following command:

```text
cargo run --example minimal --release
```

## Additional web examples

1. Install [cargo-web]:

       $ cargo install -f cargo-web

3. Go into `examples/web/minimal` and start the example using one of these commands:

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

## Build and run documenation

You can build and run the latest documentation y executing the following command:

```text
cargo doc --no-deps --open
```

## Dependencies

* [orbgl_api](https://gitlab.redox-os.org/redox-os/orbgl/orbgl_api): base orbgl api