
#[cfg(target_arch = "wasm32")]
pub use stdweb::web::html_element::ImageElement as Image;

#[cfg(not(target_arch = "wasm32"))]
pub use orbimage::Image;