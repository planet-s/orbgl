#[cfg(target_arch = "wasm32")]
pub struct Font {}

#[cfg(not(target_arch = "wasm32"))]
pub use orbfont::Font;