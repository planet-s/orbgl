use orbgl_api::Canvas;

use crate::structs::{Border, Bordered, Brush, Position, Rect, Size, Thickness};

use super::Shape;

/// Used to build a rectangle, specifying additional details.
#[derive(Default)]
pub struct RectangleBuilder {
    background: Brush,
    rect: Rect,
    border: Border,
}

impl RectangleBuilder {
    /// Creates a new `RectangleBuilder` with default values.
    pub fn new() -> Self {
        RectangleBuilder::default()
    }

    /// Inserts a new background brush.
    pub fn background<B: Into<Brush>>(mut self, background: B) -> Self {
        self.background = background.into();
        self
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

    /// Inserts a new border.
    pub fn border(mut self, border: Border) -> Self {
        self.border = border;
        self
    }

    /// Builds the rectangle.
    pub fn build(self) -> Rectangle {
        Rectangle {
            rect: self.rect,
            border: self.border,
            background: self.background,
        }
    }
}

/// The `Rectangle` is used to display a rectangle on the screen.
#[derive(Default)]
pub struct Rectangle {
    rect: Rect,
    border: Border,
    background: Brush,
}

impl Rectangle {
    /// Creates a new `RectangleBuilder` object with default values.
    pub fn create() -> RectangleBuilder {
        RectangleBuilder::new()
    }

    /// Gets the background brush.
    pub fn background(&self) -> &Brush {
        &self.background
    }

    /// Sets the background brush.
    pub fn set_background<B: Into<Brush>>(&mut self, background: B) {
        self.background = background.into();
    }

    // Renders rectangle without border and radius.
    fn render_rect_path(
        &mut self,
        canvas: &mut Canvas,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        brush: Brush,
    ) {
        match brush {
            Brush::SolidColor(color) => canvas.set_fill_style(color),
            _ => {} // todo: gradient
        }

        canvas.fill_rect(x, y, width, height);
    }

    // Renders rectangle with border and without radius.
    fn render_bordered_rect_path(&mut self, canvas: &mut Canvas) {
        // border
        self.render_rect_path(
            canvas,
            self.rect.x,
            self.rect.y,
            self.rect.width,
            self.rect.height,
            self.border.brush().clone(),
        );

        // content
        self.render_rect_path(
            canvas,
            self.rect.x + self.border.thickness().left,
            self.rect.y + self.border.thickness().top,
            self.rect.width - self.border.thickness().left - self.border.thickness().right,
            self.rect.height - self.border.thickness().top - self.border.thickness().right,
            self.background.clone(),
        );
    }

    // Builds rectangle path with radius and without border.
    fn render_rounded_rect_path(
        &mut self,
        canvas: &mut Canvas,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        radius: f64,
        brush: Brush,
    ) {
        let m_pi = 3.14159265;
        let degrees = m_pi / 180.0;

        // canvas.begin_path();
        canvas.arc(
            x + width - radius,
            y + radius,
            radius,
            -90.0 * degrees,
            0.0 * degrees,
        );
        canvas.arc(
            x + width - radius,
            y + height - radius,
            radius,
            0.0 * degrees,
            90.0 * degrees,
        );
        canvas.arc(
            x + radius,
            y + height - radius,
            radius,
            90.0 * degrees,
            180.0 * degrees,
        );
        canvas.arc(
            x + radius,
            y + radius,
            radius,
            180.0 * degrees,
            270.0 * degrees,
        );

        match brush {
            Brush::SolidColor(color) => canvas.set_fill_style(color),
            _ => {} // todo: gradient
        }

        canvas.fill();
    }

    // Renders rectangle with border and radius.
    fn render_rounded_bordered_rect_path(&mut self, canvas: &mut Canvas) {
        // border
        self.render_rounded_rect_path(
            canvas,
            self.rect.x,
            self.rect.y,
            self.rect.width,
            self.rect.height,
            self.border.radius(),
            self.border.brush().clone(),
        );

        // content
         self.render_rounded_rect_path(
            canvas,
            self.rect.x + self.border.thickness().left,
            self.rect.y + self.border.thickness().top,
            self.rect.width - self.border.thickness().left - self.border.thickness().right,
            self.rect.height - self.border.thickness().top - self.border.thickness().right,
            self.border.radius(),
            self.background.clone(),
        );
    }
}

impl Shape for Rectangle {
    fn render(&mut self, canvas: &mut Canvas) {
         let has_thickness = self.border.thickness().left > 0.0
            || self.border.thickness().top > 0.0
            || self.border.thickness().right > 0.0
            || self.border.thickness().bottom > 0.0;

        if self.border.radius() > 0.0 {
            if has_thickness {
                self.render_rounded_bordered_rect_path(canvas);
            } else {
                self.render_rounded_rect_path(
                    canvas,
                    self.rect.x,
                    self.rect.y,
                    self.rect.width,
                    self.rect.height,
                    self.border.radius(),
                    self.background.clone(),
                );
            }
        } else {
            if has_thickness {
                self.render_bordered_rect_path(canvas);
            } else {
                self.render_rect_path(
                    canvas,
                    self.rect.x,
                    self.rect.y,
                    self.rect.width,
                    self.rect.height,
                    self.background.clone(),
                );
            }
        }
    }
}

impl Size for Rectangle {
    fn set_width(&mut self, width: f64) {
        self.rect.width = width;
    }

    fn width(&self) -> f64 {
        self.rect.width
    }

    fn set_height(&mut self, height: f64) {
        self.rect.height = height;
    }

    fn height(&self) -> f64 {
        self.rect.height
    }

    fn set_size(&mut self, width: f64, height: f64) {
        self.rect.width = width;
        self.rect.height = height;
    }

    fn size(&self) -> (f64, f64) {
        (self.rect.width, self.rect.height)
    }
}

impl Position for Rectangle {
    fn set_x(&mut self, x: f64) {
        self.rect.x = x;
    }

    fn x(&self) -> f64 {
        self.rect.x
    }

    fn set_y(&mut self, y: f64) {
        self.rect.y = y;
    }

    fn y(&self) -> f64 {
        self.rect.y
    }

    fn set_position(&mut self, x: f64, y: f64) {
        self.rect.x = x;
        self.rect.y = y;
    }

    fn position(&self) -> (f64, f64) {
        (self.rect.x, self.rect.y)
    }
}

impl Bordered for Rectangle {
    fn border_thickness(&self) -> Thickness {
        self.border.thickness()
    }

    fn set_border_thickness(&mut self, thickness: Thickness) {
        self.border.set_thickness(thickness);
    }

    fn border_brush(&self) -> &Brush {
        &self.border.brush()
    }

    fn set_border_brush(&mut self, brush: Brush) {
        self.border.set_brush(brush);
    }

    fn border_radius(&self) -> f64 {
        self.border.radius()
    }

    fn set_border_radius(&mut self, radius: f64) {
        self.border.set_radius(radius);
    }

    fn border(&self) -> &Border {
        &self.border
    }

    fn set_border(&mut self, border: Border) {
        self.border = border;
    }
}
