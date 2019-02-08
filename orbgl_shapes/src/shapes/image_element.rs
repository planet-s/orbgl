#[cfg(not(target_arch = "wasm32"))]
use orbclient::Renderer;

use crate::structs::{Position, Rect, Size};

use super::Shape;
use orbgl_api::{Canvas, Color, Image, FromSource};

/// Used to build a image element, specifying additional details.
pub struct ImageElementBuilder {
    pub source: String,
    pub rect: Rect,
    pub source_rect: Option<Rect>,
}

impl ImageElementBuilder {
    /// Creates a new image builder with the given image `source`.
    pub fn new<S: Into<String>>(source: S) -> Self {
        ImageElementBuilder {
            source: source.into(),
            rect: Rect::default(),
            source_rect: None,
        }
    }

    /// Inserts a new position.
    pub fn position(mut self, x: f64, y: f64) -> Self {
        self.rect.x = x;
        self.rect.y = y;
        self
    }

    /// Inserts a new size.
    pub fn size(mut self, width: f64, height: f64) -> Self {
        self.rect.width = width;
        self.rect.height = height;
        self
    }

    /// Inserts a new bounding rect and overwrites position and size.
    pub fn rect(self, x: f64, y: f64, width: f64, height: f64) -> Self {
        self.position(x, y).size(width, height)
    }

    /// Inserts a new source rect.
    pub fn source_rect(
        mut self,
        source_x: f64,
        source_y: f64,
        source_width: f64,
        source_height: f64,
    ) -> Self {
        self.source_rect = Some(Rect {
            x: source_x,
            y: source_y,
            width: source_width,
            height: source_height,
        });
        self
    }

    /// Builds the image element.
    pub fn build(self) -> ImageElement {
        let mut image = None;
        let mut inner_size = None;
        let source = self.source.clone();

        if let Ok(img) = Image::from_source(&self.source) {
            inner_size = Some((img.width() as f64, img.height() as f64));
            image = Some(img);
        }

        ImageElement {
            rect: self.rect,
            inner_size,
            source,
            source_rect: self.source_rect,
            image,
        }
    }
}

/// The `ImageElement` is used to display a image on the screen.
#[derive(Clone)]
pub struct ImageElement {
    rect: Rect,
    inner_size: Option<(f64, f64)>,
    source: String,
    source_rect: Option<Rect>,
    image: Option<Image>,
}

impl ImageElement {
    /// Creates a new `ImageBuilder` object.
    pub fn create<S: Into<String>>(source: S) -> ImageElementBuilder {
        ImageElementBuilder::new(source)
    }

    /// Gets the file source.
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Sets the file source.
    pub fn set_source<S: Into<String>>(&mut self, source: S) {
        self.source = source.into();
        self.set_image(self.source.clone());   
    }

    /// Gets the source rect.
    pub fn source_rect(&self) -> &Option<Rect> {
        &self.source_rect
    }

    /// Sets the source rect.
    pub fn set_source_rect(&mut self, source_rect: Rect) {
        self.source_rect = Some(source_rect);
    }

    // Sets the inner image.
    fn set_image<S: Into<String>>(&mut self, source: S) {
        if let Ok(img) = Image::from_source(&source.into()) {
            self.inner_size = Some((img.width() as f64, img.height() as f64));
            self.image = Some(img);
        } else {
            self.inner_size = None;
        }
    }
}

impl Shape for ImageElement {
    fn render(&mut self, canvas: &mut Canvas) {
        if let Some(image) = &mut self.image {
            if let Some(source_rect) = self.source_rect {
                if self.rect.width == 0.0 && self.rect.height == 0.0 {
                    let image_width = image.width() as f64;
                    let image_height = image.height() as f64;

                    canvas.draw_image_with_clip_and_size(
                        image,
                        source_rect.x,
                        source_rect.y,
                        source_rect.width,
                        source_rect.height,
                        self.rect.x,
                        self.rect.y,
                        image_width,
                        image_height,
                    );
                } else {
                    canvas.draw_image_with_clip_and_size(
                        image,
                        source_rect.x,
                        source_rect.y,
                        source_rect.width,
                        source_rect.height,
                        self.rect.x,
                        self.rect.y,
                        self.rect.width,
                        self.rect.height,
                    );
                }
            } else {
                if self.rect.width == 0.0 && self.rect.height == 0.0 {
                    canvas.draw_image(image, self.rect.x, self.rect.y);
                } else {
                    canvas.draw_image_with_size(
                        image,
                        self.rect.x,
                        self.rect.y,
                        self.rect.width,
                        self.rect.height,
                    );
                }
            }
        } else {
            // placeholder border if image could not be loaded
            canvas.set_stroke_style(Color::rgb(0, 0, 0));
            canvas.rect(self.x(), self.y(), self.width(), self.height());
            canvas.stroke();
        }
    }
}

impl Size for ImageElement {
    fn width(&self) -> f64 {
        if let Some(size) = self.inner_size {
            return size.0;
        }

        self.rect.width
    }

    fn set_width(&mut self, width: f64) {
        self.rect.width = width;
    }

    fn height(&self) -> f64 {
        if let Some(size) = self.inner_size {
            return size.1;
        }
        self.rect.height
    }

    fn set_height(&mut self, height: f64) {
        self.rect.height = height;
    }

    fn size(&self) -> (f64, f64) {
        (self.rect.width, self.rect.height)
    }

    fn set_size(&mut self, width: f64, height: f64) {
        self.rect.width = width;
        self.rect.height = height;
    }
}

impl Position for ImageElement {
    fn x(&self) -> f64 {
        self.rect.x
    }

    fn set_x(&mut self, x: f64) {
        self.rect.x = x;
    }

    fn y(&self) -> f64 {
        self.rect.y
    }

    fn set_y(&mut self, y: f64) {
        self.rect.y = y;
    }

    fn position(&self) -> (f64, f64) {
        (self.rect.x, self.rect.y)
    }

    fn set_position(&mut self, x: f64, y: f64) {
        self.rect.x = x;
        self.rect.y = y;
    }
}
