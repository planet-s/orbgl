#[cfg(target_arch = "wasm32")]
pub use stdweb::web::html_element::ImageElement as Image;

#[cfg(target_arch = "wasm32")]
impl FromSource for Image {
    fn from_source(path: &str) -> Result<Image, String> {
        let mut image = Image::new();
        image.set_src(path);
        Ok(image)
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use orbimage::Image;

#[cfg(not(target_arch = "wasm32"))]
impl FromSource for Image {
    fn from_source(path: &str) -> Result<Image, String> {
        Image::from_path(path)
    }
}

pub trait FromSource {
    fn from_source(path: &str) -> Result<Image, String>;
}
