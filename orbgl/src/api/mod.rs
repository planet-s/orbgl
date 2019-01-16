pub use self::canvas::*;
mod canvas;

#[cfg(target_arch = "wasm32")]
pub use self::color::*;

#[cfg(not(target_arch = "wasm32"))]
pub use orbclient::Color;

#[cfg(not(target_arch = "wasm32"))]
pub use orbimage::Image as Image;

#[cfg(target_arch = "wasm32")]
pub use stdweb::web::html_element::ImageElement as Image;

#[cfg(target_arch = "wasm32")]
mod color;

