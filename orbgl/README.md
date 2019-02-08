# OrbGL - The Orbital Graphics Library

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./../LICENSE)

This is the main crate of the Orbital Graphics Library (OrbGL). OrbGL is a library for rendering 2D and 3D graphics.

## Minimal Example

```rust
use orbclient::{Window, EventOption, Renderer};
use orbgl::prelude::*;

fn main() {
    let w = 800;
    let h = 600;
    let (width, height) = orbclient::get_display_size().unwrap();
    let mut window = Window::new_flags((width as i32) / 4,
                                       (height as i32) / 4,
                                       w,
                                       h,
                                       "OrbGL", &[orbclient::WindowFlag::Async]).unwrap();
    let (win_w, win_h) = (w, h);
    window.rect(0, 0, win_w, win_h, Color::rgb(255, 255, 255));

    let surface = FramebufferSurface::new(800, 600, window.data_mut().as_mut_ptr() as *mut u8);
    let render_engine = OrbGLRenderEngine::new(surface.clone()); 
    let mut canvas = Canvas::new(render_engine.clone());

    canvas.set_fill_style(Color::rgba(0, 0, 0, 255));
    canvas.fill_rect(10.0, 10.0, 100.0, 75.0);
    
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
cargo run --example widgets --release
```

## Build and run documentation

You can build and run the latest documentation y executing the following command:

```text
cargo doc --no-deps --open
```

## Use OrbGL with cairo

If you want to use OrbGL with cairo you have to install cairo graphics. Otherwise you have to build OrbGL with the "plain" feature.

* With Ubuntu, please to type ```sudo apt-get install libcairo2-dev``` in your console.
* With macOS and homebrew, please to type ```brew install cairo``` in your console.
* With macOS and macports, please to type ```sudo port install cairo``` in your console.

## Dependencies

* [orbgl_api](https://gitlab.redox-os.org/redox-os/orbgl/orbgl_api): base orbgl api
* [rust_cairo (optional)](https://gitlab.redox-os.org/redox-os/rust-cairo): drawing with cairo