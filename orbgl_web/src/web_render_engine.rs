use std::{cell::RefCell, rc::Rc};

use stdweb::web::FillRule;

use stdweb::{_js_impl, js};

use orbgl_api::{Color, Image, RenderEngine};

use crate::web_surface::WebSurface;

// use crate::{
//     api::{Color, Image},
//     render_engine::RenderEngine,
// };

pub struct WebRenderEngine {
    surface: WebSurface,
}

impl WebRenderEngine {
    pub fn new(surface: WebSurface) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(WebRenderEngine { surface }))
    }
}

impl RenderEngine for WebRenderEngine {
    fn save(&mut self) {
        self.surface.context.save();
    }

    fn restore(&mut self) {
        self.surface.context.restore();
    }

    fn fill(&mut self) {
        self.surface.context.fill(FillRule::default());
    }

    fn stroke(&mut self) {
        self.surface.context.stroke();
    }

    fn begin_path(&mut self) {
        self.surface.context.begin_path();
    }

    fn close_path(&mut self) {
        self.surface.context.close_path();
    }

    fn arc(&mut self, x: f64, y: f64, radius: f64, start_segment: f64, end_segment: f64) {
        self.surface
            .context
            .arc(x, y, radius, start_segment, end_segment, false);
    }

    fn move_to(&mut self, x: f64, y: f64) {
        self.surface.context.move_to(x, y);
    }

    fn line_to(&mut self, x: f64, y: f64) {
        self.surface.context.line_to(x, y);
    }

    /*
    fn quadratic_curve_to(&mut self, cpx: f64, cpy: f64, x: f64, y: f64) {
        unsafe {
        }
    }
    */

    fn bezier_curve_to(&mut self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
        self.surface
            .context
            .bezier_curve_to(cp1x, cp1y, cp2x, cp2y, x, y);
    }

    fn rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.surface.context.rect(x, y, width, height);
    }

    fn fill_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.surface.context.fill_rect(x, y, width, height);
    }

    fn stroke_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.surface.context.stroke_rect(x, y, width, height);
    }

    fn clear_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.surface.context.clear_rect(x, y, width, height);
    }

    fn scale(&mut self, sx: f64, sy: f64) {
        self.surface.context.scale(sx, sy);
    }

    fn rotate(&mut self, angle: f64) {
        self.surface.context.rotate(angle);
    }

    fn translate(&mut self, tx: f64, ty: f64) {
        self.surface.context.translate(tx, ty);
    }

    fn transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        self.surface.context.transform(a, b, c, d, e, f);
    }

    fn set_transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        self.surface.context.set_transform(a, b, c, d, e, f);
    }

    fn set_fill_style(&mut self, color: Color) {
        self.surface
            .context
            .set_fill_style_color(&color.to_string());
    }

    fn set_stroke_style(&mut self, color: Color) {
        self.surface
            .context
            .set_stroke_style_color(&color.to_string());
    }

    fn set_line_width(&mut self, line_width: f64) {
        self.surface.context.set_line_width(line_width);
    }

    fn draw_image(&mut self, image: &mut Image, x: f64, y: f64) {
        let image = image.clone();
        js! {
            var img = @{&image};
            var context = @{&self.surface.context};
            img.onload = function() {
                context.drawImage(@{&image}, @{&x}, @{&y});
            };
        }
    }

    fn draw_image_with_size(&mut self, image: &mut Image, x: f64, y: f64, width: f64, height: f64) {
        let image = image.clone();
        js! {
            console.log("image");
            var img = @{&image};
            var context = @{&self.surface.context};
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
            var context = @{&self.surface.context};
            img.onload = function() {
                console.log("test");
                context.drawImage(@{&image}, @{&clip_x}, @{&clip_y}, @{&clip_width}, @{&clip_height}, @{&x}, @{&y}, @{&width}, @{&height});
            };
        }
    }

    fn clip(&mut self) {
        self.surface.context.clip(FillRule::default());
    }
}