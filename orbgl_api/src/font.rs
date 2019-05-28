#[cfg(target_arch = "wasm32")]
#[derive(Copy, Clone, Default, Debug)]
pub struct Font {}

#[cfg(not(target_arch = "wasm32"))]
pub use orbfont::Font;