
use std::{
    rc::Rc,
    cell::RefCell,
};

use crate::{ Color, Image, RenderEngine};

/// Is used to draw graphics.
pub struct Canvas {
    pub render_engine: Rc<RefCell<RenderEngine>>,
}

impl Canvas {
    /// Creates a new canvas with a render engine.
    pub fn new(render_engine: Rc<RefCell<RenderEngine>>) -> Self {
        Self {
            render_engine: render_engine,
        }
    }

    /// Saves the entire state of the canvas by pushing the current state onto a stack.
    pub fn save(&mut self) {
        self.render_engine.borrow_mut().save();
    }

    /// Restores the most recently saved canvas state by popping the top entry in the drawing state stack. If there is no saved state, this method does nothing.
    pub fn restore(&mut self) {
        self.render_engine.borrow_mut().restore();
    }

    /// Fills the current or given path with the current file style.
    pub fn fill(&mut self) {
        self.render_engine.borrow_mut().fill();
    }

    /// Strokes {outlines} the current or given path with the current stroke style.
    pub fn stroke(&mut self) {
        self.render_engine.borrow_mut().stroke();
    }

    /// Starts a new path by emptying the list of sub-paths. Call this when you want to create a new path.
    pub fn begin_path(&mut self) {
        self.render_engine.borrow_mut().begin_path();
    }

    /// Attempts to add a straight line from the current point to the start of the current sub-path. If the shape has already been closed or has only one point, this function does nothing.
    pub fn close_path(&mut self) {
        self.render_engine.borrow_mut().close_path();
    }

    /// Creates a circular arc centered at (x, y) with a radius of radius. The path starts at startAngle and ends at endAngle.
    pub fn arc(&mut self, x: f64, y: f64, radius: f64, start_segment: f64, end_segment: f64) {
        self.render_engine.borrow_mut().arc(x, y, radius, start_segment, end_segment);
    }

    /// Begins a new sub-path at the point specified by the given {x, y} coordinates.
    pub fn move_to(&mut self, x: f64, y: f64) {
        self.render_engine.borrow_mut().move_to(x, y);
    }

    /// Adds a straight line to the current sub-path by connecting the sub-path's last point to the specified {x, y} coordinates.
    pub fn line_to(&mut self, x: f64, y: f64) {
        self.render_engine.borrow_mut().line_to(x, y);
    }

    /// Adds a quadratic Bézier curve to the current sub-path.
    pub fn quadratic_curve_to(&mut self, cpx: f64, cpy: f64, x: f64, y: f64) {
        self.render_engine.borrow_mut().quadratic_curve_to(cpx, cpy, x, y);
    }

    /// Adds a cubic Bézier curve to the current sub-path. It requires three points: the first two are control points and the third one is the end point. The starting point is the latest point in the current path, which can be changed using MoveTo{} before creating the Bézier curve.
    pub fn bezier_curve_to(&mut self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
        self.render_engine.borrow_mut().bezier_curve_to(cp1x, cp1y, cp2x, cp2y, x, y);
    }

    /// Adds a rectangle to the current path.
    pub fn rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.render_engine.borrow_mut().rect(x, y, width, height);
    }

    /// Draws a filled rectangle whose starting point is at the coordinates {x, y} with the specified width and height and whose style is determined by the fillStyle attribute.
    pub fn fill_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.render_engine.borrow_mut().fill_rect(x, y, width, height);
    }

    /// Draws a rectangle that is stroked (outlined) according to the current strokeStyle and other context settings.
    pub fn stroke_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.render_engine.borrow_mut().stroke_rect(x, y, width, height);
    }

    /// Erases the pixels in a rectangular area by setting them to transparent black.
    pub fn clear_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.render_engine.borrow_mut().clear_rect(x, y, width, height);
    }

    /// Adds a scaling transformation to the canvas units horizontally and/or vertically.
    pub fn scale(&mut self, sx: f64, sy: f64) {
        self.render_engine.borrow_mut().scale(sx, sy);
    }

    /// Adds a rotation to the transformation matrix.
    pub fn rotate(&mut self, angle: f64) {
        self.render_engine.borrow_mut().rotate(angle);
    }

    /// Adds a translation transformation to the current matrix.
    pub fn translate(&mut self, tx: f64, ty: f64) {
        self.render_engine.borrow_mut().translate(tx, ty);
    }

    /// Multiplies the current transformation with the matrix described by the arguments of this method. You are able to scale, rotate, move and skew the context.
    pub fn transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        self.render_engine.borrow_mut().transform(a, b, c, d, e, f);
    }

    /// Sets the tranformation.
    pub fn set_transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        self.render_engine.borrow_mut().set_transform(a, b, c, d, e, f);
    }

    /// Specifies the fill color to use inside shapes.
    pub fn set_fill_style(&mut self, color: Color) {
        self.render_engine.borrow_mut().set_fill_style(color);
    }

    /// Specifies the fill stroke to use inside shapes.
    pub fn set_stroke_style(&mut self, color: Color) {
        self.render_engine.borrow_mut().set_stroke_style(color);
    }

    /// Sets the thickness of lines.
    pub fn set_line_width(&mut self, line_width: f64) {
        self.render_engine.borrow_mut().set_line_width(line_width);
    }

    /// Draws the image.
    pub fn draw_image(&mut self, image: &mut Image, x: f64, y: f64) {
        self.render_engine.borrow_mut().draw_image(image, x, y);
    }

    /// Draws the image with the given size.
    pub fn draw_image_with_size(&mut self, image: &mut Image, x: f64, y: f64, width: f64, height: f64) {
        self.render_engine.borrow_mut().draw_image_with_size(image, x, y, width, height);
    }

    /// Draws the given part of the image.
    pub fn draw_image_with_clip_and_size(&mut self, image: &mut Image, clip_x: f64, clip_y: f64, clip_width: f64, clip_height: f64, x: f64, y: f64, width: f64, height: f64) {
        self.render_engine.borrow_mut().draw_image_with_clip_and_size(image,clip_x, clip_y,clip_width,clip_height,x, y, width, height);
    }
}