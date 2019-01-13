use orbclient::Color;
use std::rc::Rc;
use std::cell::RefCell;
use super::super::Surface;

pub trait RenderEngine {
    fn save(&mut self) { println!("RenderEngine: 'save' is not implemented."); }
    fn restore(&mut self) { println!("RenderEngine: 'restore' is not implemented."); }
    fn fill(&mut self) { println!("RenderEngine: 'fill' is not implemented."); }
    fn stroke(&mut self) { println!("RenderEngine: 'stroke' is not implemented."); }
    fn begin_path(&mut self) { println!("RenderEngine: 'begin_path' is not implemented."); }
    fn close_path(&mut self) { println!("RenderEngine: 'close_path' is not implemented."); }
    fn arc(&mut self, x: f64, y: f64, radius: f64, start_segment: f64, end_segment: f64) { println!("RenderEngine: 'arc' is not implemented."); }
    fn move_to(&mut self, x: f64, y: f64) { println!("RenderEngine: 'move_to' is not implemented."); }
    fn line_to(&mut self, x: f64, y: f64) { println!("RenderEngine: 'line_to' is not implemented."); }
    fn quadratic_curve_to(&mut self, cpx: f64, cpy: f64, x: f64, y: f64) { println!("RenderEngine: 'quadratic_curve_to' is not implemented."); }
    fn bezier_curve_to(&mut self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) { println!("RenderEngine: 'bezier_curve_to' is not implemented."); }
    fn rect(&mut self, x: f64, y: f64, width: f64, height: f64) { println!("RenderEngine: 'rect' is not implemented."); }
    fn fill_rect(&mut self, x: f64, y: f64, width: f64, height: f64) { println!("RenderEngine: 'fill_rect' is not implemented."); }
    fn stroke_rect(&mut self, x: f64, y: f64, width: f64, height: f64) { println!("RenderEngine: 'stroke_rect' is not implemented."); }
    fn clear_rect(&mut self, x: f64, y: f64, width: f64, height: f64) { println!("RenderEngine: 'clear_rect' is not implemented."); }
    fn scale(&mut self, sx: f64, sy: f64) { println!("RenderEngine: 'scale' is not implemented."); }
    fn rotate(&mut self, angle: f64) { println!("RenderEngine: 'rotate' is not implemented."); }
    fn translate(&mut self, tx: f64, ty: f64) { println!("RenderEngine: 'translate' is not implemented."); }
    fn transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) { println!("RenderEngine: 'transform' is not implemented."); }
    fn set_transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) { println!("RenderEngine: 'set_transform' is not implemented."); }
    fn set_fill_style(&mut self, color: Color) { println!("RenderEngine: 'set_fill_style' is not implemented."); }
    fn set_stroke_style(&mut self, color: Color) { println!("RenderEngine: 'set_stroke_style' is not implemented."); }
    fn set_line_width(&mut self, line_width: f64) { println!("RenderEngine: 'set_line_width' is not implemented."); }

    fn pixel(&mut self, x: i32, y: i32, color: Color) { println!("RenderEngine: 'pixel' is not implemented."); }
    fn set_surface(&mut self, surface: Rc<RefCell<Surface>>) { println!("RenderEngine: 'set_surface' is not implemented."); }
}







