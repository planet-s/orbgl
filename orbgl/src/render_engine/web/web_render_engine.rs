use std::{cell::RefCell, rc::Rc};

use stdweb::web::{CanvasRenderingContext2d, FillRule};

use stdweb::{
    _js_impl, js,
};

use crate::{
    api::{Color, Image},
    render_engine::RenderEngine,
};

pub struct WebRenderEngine {
    context: CanvasRenderingContext2d,
}

impl WebRenderEngine {
    pub fn new(context: CanvasRenderingContext2d) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(WebRenderEngine { context }))
    }
}

impl RenderEngine for WebRenderEngine {
    fn save(&mut self) {
        self.context.save();
    }

    fn restore(&mut self) {
        self.context.restore();
    }

    fn fill(&mut self) {
        self.context.fill(FillRule::default());
    }

    fn stroke(&mut self) {
        self.context.stroke();
    }

    fn begin_path(&mut self) {
        self.context.begin_path();
    }

    fn close_path(&mut self) {
        self.context.close_path();
    }

    fn arc(&mut self, x: f64, y: f64, radius: f64, start_segment: f64, end_segment: f64) {
        self.context
            .arc(x, y, radius, start_segment, end_segment, false);
    }

    fn move_to(&mut self, x: f64, y: f64) {
        self.context.move_to(x, y);
    }

    fn line_to(&mut self, x: f64, y: f64) {
        self.context.line_to(x, y);
    }

    /*
    fn quadratic_curve_to(&mut self, cpx: f64, cpy: f64, x: f64, y: f64) {
        unsafe {
        }
    }
    */

    fn bezier_curve_to(&mut self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
        self.context.bezier_curve_to(cp1x, cp1y, cp2x, cp2y, x, y);
    }

    fn rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.context.rect(x, y, width, height);
    }

    fn fill_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.context.fill_rect(x, y, width, height);
    }

    fn stroke_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.context.stroke_rect(x, y, width, height);
    }

    fn clear_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.context.clear_rect(x, y, width, height);
    }

    fn scale(&mut self, sx: f64, sy: f64) {
        self.context.scale(sx, sy);
    }

    fn rotate(&mut self, angle: f64) {
        self.context.rotate(angle);
    }

    fn translate(&mut self, tx: f64, ty: f64) {
        self.context.translate(tx, ty);
    }

    fn transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        self.context.transform(a, b, c, d, e, f);
    }

    fn set_transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        self.context.set_transform(a, b, c, d, e, f);
    }

    fn set_fill_style(&mut self, color: Color) {
        self.context.set_fill_style_color(&color.to_string());
    }

    fn set_stroke_style(&mut self, color: Color) {
        self.context.set_stroke_style_color(&color.to_string());
    }

    fn set_line_width(&mut self, line_width: f64) {
        self.set_line_width(line_width);
    }

    fn draw_image(&mut self, image: &mut Image, x: f64, y: f64) {
        let image = image.clone();
        js! {
            var img = @{&image};
            var context = @{&self.context};
            img.onload = function() {
                context.drawImage(@{&image}, @{&x}, @{&y});
            };
        }
    }

    fn draw_image_with_size(&mut self, image: &mut Image, x: f64, y: f64, width: f64, height: f64) {
        let image = image.clone();
        js! {
            var img = @{&image};
            var context = @{&self.context};
            img.onload = function() {
                console.log("test");
                context.drawImage(@{&image}, @{&x}, @{&y}, @{&width}, @{&height});
            };
        }
    }

    fn draw_image_with_clip_and_size(
        &mut self,
        image: &mut Image,
        clip_x: f64,
        clip_y: f64,
        clip_width: f64,
        clip_height: f64,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) {
        let image = image.clone();
        js! {
            var img = @{&image};
            var context = @{&self.context};
            img.onload = function() {
                console.log("test");
                context.drawImage(@{&image}, @{&clip_x}, @{&clip_y}, @{&clip_width}, @{&clip_height}, @{&x}, @{&y}, @{&width}, @{&height});
            };
        }
    }
}
