use std::path::Path;

#[cfg(target_arch = "wasm32")]
#[derive(Clone, Default, Debug)]
pub struct Font {
    pub family: String
}

impl Font {
    pub fn from_family(family: impl Into<String>) -> Font {
        Font {
            family: family.into()
        }  
    }
    pub fn from_data(_: Box<[u8]>) -> Result<Font, String> {
        Ok(Font { family: String::default() })
    }

    pub fn from_path<P: AsRef<Path>>(_: P) -> Result<Font, String> {
         Ok(Font {  family: String::default() })
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use orbfont::Font;