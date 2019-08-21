use std::{cell::RefCell, rc::Rc};

use rust_cairo::*;

use orbgl_api::{Color, Image, RenderEngine, Surface};

use orbclient::Renderer;

use crate::surface::ImageSurface;

use super::canvas_paint_state::CanvasPaintState;

pub struct CairoRenderEngine {
    pub surface: Rc<RefCell<Surface>>,
    pub cr_layer_a: *mut cairo_t,
    pub cr_layer_b: *mut cairo_t,
    state: CanvasPaintState,
    saved_states: Vec<CanvasPaintState>,
}

impl CairoRenderEngine {
    pub fn new(surface: Rc<RefCell<Surface>>) -> Rc<RefCell<Self>> {
        let cr_layer_a;
        let cr_layer_b;
        unsafe {
            let mut surface = surface.borrow_mut();
            let width = surface.width() as i32;
            let height = surface.height() as i32;
            let pixel_buffer_pointer = surface.data_mut().as_mut_ptr() as *mut u8;
            let surface = cairo_image_surface_create_for_data(
                pixel_buffer_pointer,
                CAIRO_FORMAT_ARGB32,
                width as i32,
                height as i32,
                cairo_format_stride_for_width(CAIRO_FORMAT_ARGB32, width as i32),
            );
            cr_layer_a = cairo_create(surface);
        }

        unsafe {
            let mut surface = surface.borrow_mut();
            let width = surface.width() as i32;
            let height = surface.height() as i32;
            let pixel_buffer_pointer = surface.data_mut().as_mut_ptr() as *mut u8;
            let surface = cairo_image_surface_create_for_data(
                pixel_buffer_pointer,
                CAIRO_FORMAT_ARGB32,
                width as i32,
                height as i32,
                cairo_format_stride_for_width(CAIRO_FORMAT_ARGB32, width as i32),
            );
            cr_layer_b = cairo_create(surface);
        }

        Rc::new(RefCell::new(Self {
            cr_layer_a: cr_layer_a,
            cr_layer_b: cr_layer_b,
            surface: surface,
            state: CanvasPaintState::new(),
            saved_states: vec![],
        }))
    }
}

impl RenderEngine for CairoRenderEngine {
    fn save(&mut self) {
        self.saved_states.push(self.state);
        unsafe {
            cairo_save(self.cr_layer_a);
            cairo_save(self.cr_layer_b);
        }
    }

    fn restore(&mut self) {
        if self.saved_states.len() > 0 {
            self.state = self.saved_states.pop().unwrap();
        }
        unsafe {
            cairo_restore(self.cr_layer_a);
            cairo_restore(self.cr_layer_b);
        }
    }

    fn fill(&mut self) {
        unsafe {
            cairo_set_source_rgba(
                self.cr_layer_a,
                self.state.fill_style.r() as f64 / 255.0,
                self.state.fill_style.g() as f64 / 255.0,
                self.state.fill_style.b() as f64 / 255.0,
                self.state.fill_style.a() as f64 / 255.0,
            );
            cairo_fill(self.cr_layer_a);
        }
    }

    fn stroke(&mut self) {
        unsafe {
            cairo_set_source_rgba(
                self.cr_layer_a,
                self.state.stroke_style.r() as f64 / 255.0,
                self.state.stroke_style.g() as f64 / 255.0,
                self.state.stroke_style.b() as f64 / 255.0,
                self.state.stroke_style.a() as f64 / 255.0,
            );
            cairo_set_line_width(self.cr_layer_a, self.state.line_width);
            cairo_stroke(self.cr_layer_a)
        }
    }

    fn begin_path(&mut self) {
        unsafe {
            //cairo_new_path(self.cr_layer_a);
            cairo_new_sub_path(self.cr_layer_a);
        }
    }

    fn close_path(&mut self) {
        unsafe {
            cairo_close_path(self.cr_layer_a);
        }
    }

    fn arc(&mut self, x: f64, y: f64, radius: f64, start_segment: f64, end_segment: f64) {
        unsafe {
            cairo_arc(self.cr_layer_a, x, y, radius, start_segment, end_segment);
        }
    }

    fn move_to(&mut self, x: f64, y: f64) {
        unsafe {
            cairo_move_to(self.cr_layer_a, x, y);
        }
    }

    fn line_to(&mut self, x: f64, y: f64) {
        unsafe {
            cairo_line_to(self.cr_layer_a, x, y);
        }
    }

    /*
    fn quadratic_curve_to(&mut self, cpx: f64, cpy: f64, x: f64, y: f64) {
        unsafe {
        }
    }
    */

    fn bezier_curve_to(&mut self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
        unsafe { cairo_curve_to(self.cr_layer_a, cp1x, cp1y, cp2x, cp2y, x, y) }
    }

    fn rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        unsafe {
            cairo_rectangle(self.cr_layer_a, x, y, width, height);
        }
    }

    fn fill_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        unsafe {
            cairo_rectangle(self.cr_layer_b, x, y, width, height);
            cairo_set_source_rgba(
                self.cr_layer_b,
                self.state.fill_style.r() as f64 / 255.0,
                self.state.fill_style.g() as f64 / 255.0,
                self.state.fill_style.b() as f64 / 255.0,
                self.state.fill_style.a() as f64 / 255.0,
            );
            cairo_fill(self.cr_layer_b);
        }
    }

    fn stroke_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        unsafe {
            cairo_rectangle(self.cr_layer_b, x, y, width, height);
            cairo_set_source_rgba(
                self.cr_layer_b,
                self.state.stroke_style.r() as f64 / 255.0,
                self.state.stroke_style.g() as f64 / 255.0,
                self.state.stroke_style.b() as f64 / 255.0,
                self.state.stroke_style.a() as f64 / 255.0,
            );
            cairo_set_line_width(self.cr_layer_b, self.state.line_width);
            cairo_stroke(self.cr_layer_b)
        }
    }

    fn clear_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        unsafe {
            cairo_save(self.cr_layer_b);
            cairo_set_operator(self.cr_layer_b, CAIRO_OPERATOR_CLEAR);
            cairo_rectangle(self.cr_layer_b, x, y, width, height);
            cairo_fill(self.cr_layer_b);
            cairo_restore(self.cr_layer_b);
        }
    }

    fn scale(&mut self, sx: f64, sy: f64) {
        unsafe {
            cairo_scale(self.cr_layer_a, sx, sy);
            cairo_scale(self.cr_layer_b, sx, sy);
        }
    }

    fn rotate(&mut self, angle: f64) {
        unsafe {
            cairo_rotate(self.cr_layer_a, angle);
            cairo_rotate(self.cr_layer_b, angle);
        }
    }

    fn translate(&mut self, tx: f64, ty: f64) {
        unsafe {
            cairo_translate(self.cr_layer_a, tx, ty);
            cairo_translate(self.cr_layer_b, tx, ty);
        }
    }

    fn transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        unsafe {
            let matrix = cairo_matrix_t {
                xx: a,
                yx: b,
                xy: c,
                yy: d,
                x0: e,
                y0: f,
            };

            cairo_transform(self.cr_layer_a, &matrix);
            cairo_transform(self.cr_layer_b, &matrix);
        }
    }

    fn set_transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        unsafe {
            let matrix = cairo_matrix_t {
                xx: a,
                yx: b,
                xy: c,
                yy: d,
                x0: e,
                y0: f,
            };
            cairo_set_matrix(self.cr_layer_a, &matrix);
            cairo_set_matrix(self.cr_layer_b, &matrix);
        }
    }

    fn set_fill_style(&mut self, color: Color) {
        self.state.fill_style = color;
    }

    fn set_stroke_style(&mut self, color: Color) {
        self.state.stroke_style = color;
    }

    fn set_line_width(&mut self, line_width: f64) {
        unsafe {
            self.state.line_width = line_width;
        }
    }

    fn draw_image(&mut self, image: &mut Image, x: f64, y: f64) {
        let width = image.width() as i32;
        let height = image.height() as i32;

        unsafe {
            let pixel_buffer_pointer = image.data_mut().as_mut_ptr() as *mut u8;
            let image_surface = cairo_image_surface_create_for_data(
                pixel_buffer_pointer,
                CAIRO_FORMAT_ARGB32,
                width as i32,
                height as i32,
                cairo_format_stride_for_width(CAIRO_FORMAT_ARGB32, width as i32),
            );
            cairo_save(self.cr_layer_a);
            cairo_translate(self.cr_layer_a, x, y);
            cairo_set_source_surface(self.cr_layer_a, image_surface, 0.0, 0.0);
            cairo_paint(self.cr_layer_a);
            cairo_restore(self.cr_layer_a);
        }
    }

    fn draw_image_with_size(&mut self, image: &mut Image, x: f64, y: f64, width: f64, height: f64) {
        let w = image.width() as i32;
        let h = image.height() as i32;
        let sx = width / image.width() as f64;
        let sy = height / image.height() as f64;

        unsafe {
            let pixel_buffer_pointer = image.data_mut().as_mut_ptr() as *mut u8;
            let image_surface = cairo_image_surface_create_for_data(
                pixel_buffer_pointer,
                CAIRO_FORMAT_ARGB32,
                w as i32,
                h as i32,
                cairo_format_stride_for_width(CAIRO_FORMAT_ARGB32, w as i32),
            );
            cairo_save(self.cr_layer_a);
            cairo_translate(self.cr_layer_a, x, y);
            cairo_scale(self.cr_layer_a, sx, sy);
            cairo_set_source_surface(self.cr_layer_a, image_surface, 0.0, 0.0);
            cairo_paint(self.cr_layer_a);
            cairo_restore(self.cr_layer_a);
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

        //create a temp clipped image
        let image_width = image.width();
        let data = image.data_mut();
        let mut clipped_image:Vec<Color> = Vec::new();
        for y in clip_y as u32..(clip_y+clip_height) as u32 {
            for x in clip_x as u32..(clip_x+clip_width) as u32 {
                let old = data[y as usize * image_width as usize + x as usize];
                clipped_image.push(old);
            }
        }

        let w = clip_width as i32;
        let h = clip_height as i32;
        let sx = width / clip_width as f64;
        let sy = height / clip_height as f64;

        unsafe {
            let pixel_buffer_pointer = clipped_image.as_mut_ptr() as *mut u8;
            let image_surface = cairo_image_surface_create_for_data(pixel_buffer_pointer, CAIRO_FORMAT_ARGB32, w as i32, h as i32, cairo_format_stride_for_width(CAIRO_FORMAT_ARGB32, w as i32));
            cairo_save(self.cr_layer_a);
            cairo_translate(self.cr_layer_a, x, y);
            cairo_rectangle(self.cr_layer_a,0.0,0.0,width, height);
            cairo_scale(self.cr_layer_a, sx, sy);
            cairo_clip(self.cr_layer_a);
            cairo_set_source_surface(self.cr_layer_a, image_surface, -clip_x, -clip_y);
            cairo_paint(self.cr_layer_a);
            cairo_restore(self.cr_layer_a);
        }

    }

    fn clip(&mut self) {
        unsafe {
            cairo_clip(self.cr_layer_a);
            cairo_clip(self.cr_layer_b);
            cairo_new_path (self.cr_layer_a);
            cairo_new_path (self.cr_layer_b);
        }
    }
}
