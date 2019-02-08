#![crate_name = "orbgl_api"]
#![crate_type = "lib"]

pub use self::canvas::Canvas;
pub use self::color::Color;
pub use self::font::Font;
pub use self::image::{Image, FromSource};
pub use self::render_engine::RenderEngine;
pub use self::surface::Surface;

mod canvas;

#[cfg(target_arch = "wasm32")]
mod color;

#[cfg(not(target_arch = "wasm32"))]
#[path = "orbclient_color.rs"]
mod color;

#[cfg(target_arch = "wasm32")]
mod font;

#[cfg(not(target_arch = "wasm32"))]
#[path = "orbfont_font.rs"]
mod font;

mod image;
pub mod prelude;
mod render_engine;
mod surface;
