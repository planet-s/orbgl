#[cfg(not(target_arch = "wasm32"))]
use orbclient::Renderer;

use crate::structs::{Position, Rect, Size};

use super::{Shape, PathSegment};
use orbgl_api::{Image, Color};

/// Used to build a image element, specifying additional details.
pub struct ImageElementBuilder {
    pub source: String,
    pub rect: Rect,
    pub source_rect: Option<Rect>,
}

impl ImageElementBuilder {
    /// Creates a new image bilder with the given image `source`.
    pub fn new<S: Into<String>>(source: S) -> Self {
        ImageElementBuilder {
            source: source.into(),
            rect: Rect::default(),
            source_rect: None,
        }
    }

    /// Inserts a new position.
    pub fn with_position(mut self, x: f64, y: f64) -> Self {
        self.rect.x = x;
        self.rect.y = y;
        self
    }

    /// Inserts a new size.
    pub fn with_size(mut self, width: f64, height: f64) -> Self {
        self.rect.width = width;
        self.rect.height = height;
        self
    }

    /// Inserts a new bounding rect and overwrites position and size.
    pub fn with_rect(self, x: f64, y: f64, width: f64, height: f64) -> Self {
        self.with_position(x, y).with_size(width, height)
    }

    /// Inserts a new source rect.
    pub fn with_source_rect(
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
        let mut image_element = ImageElement {
            path: vec![],
            rect: self.rect,
            inner_size: None,
            source: self.source,
            source_rect: self.source_rect,
        };

        image_element.build_path();
        image_element
    }
}

/// The ÌmageElement` is used to display a image on the screen.
#[derive(Clone)]
pub struct ImageElement {
    path: Vec<PathSegment>,
    rect: Rect,
    inner_size: Option<(f64, f64)>,
    source: String,
    source_rect: Option<Rect>,
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
    }

    /// Gets the source rect.
    pub fn source_rect(&self) -> &Option<Rect> {
        &self.source_rect
    }

    /// Sets the source rect.
    pub fn set_source_rect(&mut self, source_rect: Rect) {
        self.source_rect = Some(source_rect);
    }
}

impl Shape for ImageElement {
    fn path(&mut self) -> &mut [PathSegment] {
        &mut self.path
    }

    // todo: implement combi version for orbgl and orbgl_web after orbgl_api provides custom image struct
    #[cfg(target_arch = "wasm32")]
    fn build_path(&mut self) {

    }

    #[cfg(not(target_arch = "wasm32"))]
    fn build_path(&mut self) {
        self.path.clear();

        if let Ok(image) = Image::from_path(self.source()) {
            if self.width() == 0.0 && self.height() == 0.0 {
                self.inner_size = Some((image.width() as f64, image.height() as f64));
            } else {
                self.inner_size = None;
            }

            if let Some(source_rect) = self.source_rect {
                if self.width() == 0.0 && self.height() == 0.0 {
                    let image_width = image.width();
                    let image_height = image.height();

                    self.path.push(PathSegment::DrawImageWithClipAndSize {
                        image,
                        clip_x: source_rect.x,
                        clip_y: source_rect.y,
                        clip_width: source_rect.width,
                        clip_height: source_rect.height,
                        x: self.x(),
                        y: self.y(),
                        width: image_width as f64,
                        height: image_height as f64,
                    });
                } else {
                    self.path.push(PathSegment::DrawImageWithClipAndSize {
                        image,
                        clip_x: source_rect.x,
                        clip_y: source_rect.y,
                        clip_width: source_rect.width,
                        clip_height: source_rect.height,
                        x: self.x(),
                        y: self.y(),
                        width: self.width(),
                        height: self.height(),
                    });
                }
            } else {
                if self.width() == 0.0 && self.height() == 0.0 {
                    self.path.push(PathSegment::DrawImage {
                        image,
                        x: self.x(),
                        y: self.y(),
                    });
                } else {
                    self.path.push(PathSegment::DrawImageWithSize {
                        image,
                        x: self.x(),
                        y: self.y(),
                        width: self.width(),
                        height: self.height(),
                    });
                }
            }
        } else {
            // placeholder border if image could not be loaded
            self.path.push(PathSegment::SetStrokeStyleColor {
                color: Color::rgb(0, 0, 0),
            });
            self.path.push(PathSegment::Rect {
                x: self.rect.x,
                y: self.rect.y,
                width: self.rect.width,
                height: self.rect.height,
            });
            self.path.push(PathSegment::Stroke());
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
