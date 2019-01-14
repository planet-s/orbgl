use super::super::super::RenderEngine;
use std::rc::Rc;
use std::cell::RefCell;
use crate::api::{Color, Image};


pub struct Canvas {
    pub render_engine: Rc<RefCell<RenderEngine>>,
}

impl Canvas {
    pub fn new(render_engine: Rc<RefCell<RenderEngine>>) -> Self {
        Self {
            render_engine: render_engine,
        }
    }

    pub fn save(&mut self) {
        self.render_engine.borrow_mut().save();
    }
    pub fn restore(&mut self) {
        self.render_engine.borrow_mut().restore();
    }
    pub fn fill(&mut self) {
        self.render_engine.borrow_mut().fill();
    }
    pub fn stroke(&mut self) {
        self.render_engine.borrow_mut().stroke();
    }
    pub fn begin_path(&mut self) {
        self.render_engine.borrow_mut().begin_path();
    }
    pub fn close_path(&mut self) {
        self.render_engine.borrow_mut().close_path();
    }
    pub fn arc(&mut self, x: f64, y: f64, radius: f64, start_segment: f64, end_segment: f64) {
        self.render_engine.borrow_mut().arc(x, y, radius, start_segment, end_segment);
    }
    pub fn move_to(&mut self, x: f64, y: f64) {
        self.render_engine.borrow_mut().move_to(x, y);
    }
    pub fn line_to(&mut self, x: f64, y: f64) {
        self.render_engine.borrow_mut().line_to(x, y);
    }
    pub fn quadratic_curve_to(&mut self, cpx: f64, cpy: f64, x: f64, y: f64) {
        self.render_engine.borrow_mut().quadratic_curve_to(cpx, cpy, x, y);
    }
    pub fn bezier_curve_to(&mut self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
        self.render_engine.borrow_mut().bezier_curve_to(cp1x, cp1y, cp2x, cp2y, x, y);
    }
    pub fn rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.render_engine.borrow_mut().rect(x, y, width, height);
    }
    pub fn fill_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.render_engine.borrow_mut().fill_rect(x, y, width, height);
    }
    pub fn stroke_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.render_engine.borrow_mut().stroke_rect(x, y, width, height);
    }
    pub fn clear_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.render_engine.borrow_mut().clear_rect(x, y, width, height);
    }
    pub fn scale(&mut self, sx: f64, sy: f64) {
        self.render_engine.borrow_mut().scale(sx, sy);
    }
    pub fn rotate(&mut self, angle: f64) {
        self.render_engine.borrow_mut().rotate(angle);
    }
    pub fn translate(&mut self, tx: f64, ty: f64) {
        self.render_engine.borrow_mut().translate(tx, ty);
    }
    pub fn transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        self.render_engine.borrow_mut().transform(a, b, c, d, e, f);
    }
    pub fn set_transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        self.render_engine.borrow_mut().set_transform(a, b, c, d, e, f);
    }
    pub fn set_fill_style(&mut self, color: Color) {
        self.render_engine.borrow_mut().set_fill_style(color);
    }
    pub fn set_stroke_style(&mut self, color: Color) {
        self.render_engine.borrow_mut().set_stroke_style(color);
    }
    pub fn set_line_width(&mut self, line_width: f64) {
        self.render_engine.borrow_mut().set_line_width(line_width);
    }
    pub fn draw_image(&mut self, image: &mut Image, x: f64, y: f64) {
        self.render_engine.borrow_mut().draw_image(image, x, y);
    }
    pub fn draw_image_with_size(&mut self, image: &mut Image, x: f64, y: f64, width: f64, height: f64) {
        self.render_engine.borrow_mut().draw_image_with_size(image, x, y, width, height);
    }
    pub fn draw_image_with_clip_and_size(&mut self, image: &mut Image, clip_x: f64, clip_y: f64, clip_width: f64, clip_height: f64, x: f64, y: f64, width: f64, height: f64) {
        self.render_engine.borrow_mut().draw_image_with_clip_and_size(image,clip_x, clip_y,clip_width,clip_height,x, y, width, height);
    }

}