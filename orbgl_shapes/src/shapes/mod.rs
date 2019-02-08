// This private sub module contains a list of shapes.

use orbgl_api::Canvas;

pub use self::image_element::{ImageElement, ImageElementBuilder};
pub use self::rectangle::{Rectangle, RectangleBuilder};
pub use self::path_segment::PathSegment;

mod image_element;
mod rectangle;
mod path_segment;

/// Provides the base for render shapes like `Rectangle`, `ImageElement` and `Text`.
pub trait Shape {
   fn render(&mut self, canvas: &mut Canvas);
}

#[cfg(test)]
mod tests;