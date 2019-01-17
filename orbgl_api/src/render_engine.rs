use crate::{
    Color, Image
};

/// Used to implement an inner render engine for the `Canvas`.
pub trait RenderEngine {
    /// Saves the entire state of the canvas by pushing the current state onto a stack.
    fn save(&mut self) { println!("RenderEngine: 'save' is not implemented."); }

    /// Restores the most recently saved canvas state by popping the top entry in the drawing state stack. If there is no saved state, this method does nothing.
    fn restore(&mut self) { println!("RenderEngine: 'restore' is not implemented."); }

    /// Fills the current or given path with the current file style.
    fn fill(&mut self) { println!("RenderEngine: 'fill' is not implemented."); }

    /// Strokes {outlines} the current or given path with the current stroke style.
    fn stroke(&mut self) { println!("RenderEngine: 'stroke' is not implemented."); }

    /// Starts a new path by emptying the list of sub-paths. Call this when you want to create a new path.
    fn begin_path(&mut self) { println!("RenderEngine: 'begin_path' is not implemented."); }

    /// Attempts to add a straight line from the current point to the start of the current sub-path. If the shape has already been closed or has only one point, this function does nothing.
    fn close_path(&mut self) { println!("RenderEngine: 'close_path' is not implemented."); }

    /// Creates a circular arc centered at (x, y) with a radius of radius. The path starts at startAngle and ends at endAngle.
    fn arc(&mut self, _x: f64, _y: f64, _radius: f64, _start_segment: f64, _end_segment: f64) { println!("RenderEngine: 'arc' is not implemented."); }

    /// Begins a new sub-path at the point specified by the given {x, y} coordinates.
    fn move_to(&mut self, _x: f64, _y: f64) { println!("RenderEngine: 'move_to' is not implemented."); }

    /// Adds a straight line to the current sub-path by connecting the sub-path's last point to the specified {x, y} coordinates.
    fn line_to(&mut self, _x: f64, _y: f64) { println!("RenderEngine: 'line_to' is not implemented."); }

    /// Adds a quadratic Bézier curve to the current sub-path.
    fn quadratic_curve_to(&mut self, _cpx: f64, _cpy: f64, _x: f64, _y: f64) { println!("RenderEngine: 'quadratic_curve_to' is not implemented."); }
    
    /// Adds a cubic Bézier curve to the current sub-path. It requires three points: the first two are control points and the third one is the end point. The starting point is the latest point in the current path, which can be changed using MoveTo{} before creating the Bézier curve.
    fn bezier_curve_to(&mut self, _cp1x: f64, _cp1y: f64, _cp2x: f64, _cp2y: f64, _x: f64, _y: f64) { println!("RenderEngine: 'bezier_curve_to' is not implemented."); }
    
    /// Adds a rectangle to the current path.
    fn rect(&mut self, _x: f64, _y: f64, _width: f64, _height: f64) { println!("RenderEngine: 'rect' is not implemented."); }
    
    /// Draws a filled rectangle whose starting point is at the coordinates {x, y} with the specified width and height and whose style is determined by the fillStyle attribute.
    fn fill_rect(&mut self, _x: f64, _y: f64, _width: f64, _height: f64) { println!("RenderEngine: 'fill_rect' is not implemented."); }
    
    /// Draws a rectangle that is stroked (outlined) according to the current strokeStyle and other context settings.
    fn stroke_rect(&mut self, _x: f64, _y: f64, _width: f64, _height: f64) { println!("RenderEngine: 'stroke_rect' is not implemented."); }
    
    /// Erases the pixels in a rectangular area by setting them to transparent black.
    fn clear_rect(&mut self, _x: f64, _y: f64, _width: f64, _height: f64) { println!("RenderEngine: 'clear_rect' is not implemented."); }
    
    /// Adds a scaling transformation to the canvas units horizontally and/or vertically.
    fn scale(&mut self, _sx: f64, _sy: f64) { println!("RenderEngine: 'scale' is not implemented."); }
    
    /// Adds a rotation to the transformation matrix.
    fn rotate(&mut self, _angle: f64) { println!("RenderEngine: 'rotate' is not implemented."); }
    
    /// Adds a translation transformation to the current matrix.
    fn translate(&mut self, _tx: f64, _ty: f64) { println!("RenderEngine: 'translate' is not implemented."); }
    
    /// Multiplies the current transformation with the matrix described by the arguments of this method. You are able to scale, rotate, move and skew the context.
    fn transform(&mut self, _a: f64, _b: f64, _c: f64, _d: f64, _e: f64, _f: f64) { println!("RenderEngine: 'transform' is not implemented."); }
    
    /// Sets the tranformation.
    fn set_transform(&mut self, _a: f64, _b: f64, _c: f64, _d: f64, _e: f64, _f: f64) { println!("RenderEngine: 'set_transform' is not implemented."); }
    
    /// Specifies the fill color to use inside shapes.
    fn set_fill_style(&mut self, _color: Color) { println!("RenderEngine: 'set_fill_style' is not implemented."); }
    
    /// Specifies the fill stroke to use inside shapes.
    fn set_stroke_style(&mut self, _color: Color) { println!("RenderEngine: 'set_stroke_style' is not implemented."); }
    
    /// Sets the thickness of lines.
    fn set_line_width(&mut self, _line_width: f64) { println!("RenderEngine: 'set_line_width' is not implemented."); }

    /// Draws the image.
    fn draw_image(&mut self, _image: &mut Image, _x: f64, _y: f64) { println!("RenderEngine: 'draw_image' is not implemented.") }
    
    /// Draws the image with the given size.
    fn draw_image_with_size(&mut self, _image: &mut Image, _x: f64, _y: f64, _width: f64, _height: f64) { println!("RenderEngine: 'draw_image_with_size' is not implemented.") }
    
    /// Draws the given part of the image.
    fn draw_image_with_clip_and_size(&mut self, _image: &mut Image, _clip_x: f64, _clip_y: f64, _clip_width: f64, _clip_height: f64, _x: f64, _y: f64, _width: f64, _height: f64) { println!("RenderEngine: 'draw_image_with_size' is not implemented.") }
}







