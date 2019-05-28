use std::path::Path;

#[cfg(target_arch = "wasm32")]
#[derive(Copy, Clone, Default, Debug)]
pub struct Font {}

impl Font {
    pub fn from_data(_: Box<[u8]>) -> Result<Font, String> {
        Ok(Font {})
    }

    pub fn from_path<P: AsRef<Path>>(_: P) -> Result<Font, String> {
         Ok(Font {})
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use orbfont::Font;