use orbgl_api::Canvas;

use crate::{
    shapes::Shape,
};

/// Used to extends a render context and to make it easier to render a shape.
pub trait ShapeRenderer {
    /// Renders the given shape.
    fn render_shape(&mut self, shape: &mut Shape);
}

// Default Render implementation of OrbGL Canvas.
impl ShapeRenderer for Canvas {
     fn render_shape(&mut self, shape: &mut Shape) {
         shape.render(self);
    }
}
