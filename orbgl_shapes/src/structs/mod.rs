// This private submodule contains a set of basic structs used for rendering and shapes.

pub use self::border::{Border, BorderBuilder, Bordered};
pub use self::brush::{Brush, GradientStop};
pub use self::rect::{Position, Rect, Size};
pub use self::thickness::Thickness;

mod border;
mod brush;
mod rect;
mod thickness;

#[cfg(test)]
mod tests;
