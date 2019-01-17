use std::{cell::RefCell, rc::Rc};

use orbgl_api::{Color, Image, RenderEngine, Surface};

use crate::surface::ImageSurface;

use super::{
    canvas_paint_state::CanvasPaintState,
    pathbuilder::PathBuilder, point::Point,
    edge::{Edge, EdgeType},
};

pub struct OrbGLRenderEngine {
    pub surface: Rc<RefCell<Surface>>,
    path_builder: PathBuilder,
    state: CanvasPaintState,
    saved_states: Vec<CanvasPaintState>,
}

impl OrbGLRenderEngine {
    pub fn new(surface: Rc<RefCell<Surface>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            surface: surface,
            path_builder: PathBuilder::new(),
            state: CanvasPaintState::new(),
            saved_states: vec![],
        }))
    }

    fn scanline(&mut self, y: f64) -> Vec<f64> {
        let mut cross_points: Vec<f64> = Vec::new();

        let mut edges: Vec<Edge>;
        edges = self.path_builder.build();

        for edge in edges {
            let start_point = self.state.transform.apply_to_point(edge.start);
            let end_point = self.state.transform.apply_to_point(edge.end);
            let t: f64 = (((end_point.x - start_point.x) * (y as f64 - start_point.y))
                / (end_point.y - start_point.y))
                + start_point.x;
            if (start_point.y > y as f64) != (end_point.y > y as f64) {
                cross_points.push(t as f64);
            }
        }

        cross_points.sort_by(|a, b| a.partial_cmp(&b).unwrap());
        cross_points
    }

    fn pixel(&mut self, x: i32, y: i32, color: Color) {
        let mut surface = self.surface.borrow_mut();
        let w = surface.width() as i32;
        let h = surface.height() as i32;
        let data = surface.data_mut();

        if x >= 0 && y >= 0 && x < w as i32 && y < h as i32 {
            let new = color.data;
            let alpha = (new >> 24) & 0xFF;
            let old = unsafe { &mut data[y as usize * w as usize + x as usize].data };

            if self.state.override_color {
                *old = new;
            } else if alpha > 0 {
                let n_alpha = 255 - alpha;
                let rb = ((n_alpha * (*old & 0x00FF00FF)) + (alpha * (new & 0x00FF00FF))) >> 8;
                let ag = (n_alpha * ((*old & 0xFF00FF00) >> 8))
                    + (alpha * (0x01000000 | ((new & 0x0000FF00) >> 8)));

                *old = (rb & 0x00FF00FF) | (ag & 0xFF00FF00);
            }
        }
    }

    fn line(&mut self, argx1: i32, argy1: i32, argx2: i32, argy2: i32, color: Color) {
        let mut x = argx1;
        let mut y = argy1;

        let dx = if argx1 > argx2 {
            argx1 - argx2
        } else {
            argx2 - argx1
        };
        let dy = if argy1 > argy2 {
            argy1 - argy2
        } else {
            argy2 - argy1
        };

        let sx = if argx1 < argx2 { 1 } else { -1 };
        let sy = if argy1 < argy2 { 1 } else { -1 };

        let mut err = if dx > dy { dx } else { -dy } / 2;
        let mut err_tolerance;

        loop {
            self.pixel(x, y, color);
            //self.wu_circle(x,y,1,color);

            if x == argx2 && y == argy2 {
                break;
            };

            err_tolerance = 2 * err;

            if err_tolerance > -dx {
                err -= dy;
                x += sx;
            }
            if err_tolerance < dy {
                err += dx;
                y += sy;
            }
        }
    }

    /// antialiased pixel
    #[allow(unused)]
    fn aa_pixel(&mut self, x: f64, y: f64, color: Color) {
        let r = color.r();
        let g = color.g();
        let b = color.b();
        let a = color.a();

        let int_x = x as i32;
        let int_y = y as i32;
        let frac_x = x - int_x as f64;
        let frac_y = y - int_y as f64;

        let _a = (1.0 - frac_x) * (1.0 - frac_y);
        let _b = frac_x * (1.0 - frac_y);
        let _c = (1.0 - frac_x) * frac_y;
        let _d = frac_x * frac_y;

        self.pixel(int_x, int_y, Color::rgba(r, g, b, (a as f64 * _a) as u8));

        self.pixel(
            int_x + 1,
            int_y,
            Color::rgba(r, g, b, (a as f64 * _b) as u8),
        );
        self.pixel(
            int_x,
            int_y + 1,
            Color::rgba(r, g, b, (a as f64 * _c) as u8),
        );
        self.pixel(
            int_x + 1,
            int_y + 1,
            Color::rgba(r, g, b, (a as f64 * _d) as u8),
        );
    }

    /// antialiased line
    #[allow(unused)]
    fn aa_line(&mut self, x0: f64, y0: f64, x1: f64, y1: f64, color: Color) {
        let mut x0 = x0 as f64;
        let mut y0 = y0 as f64;
        let mut x1 = x1 as f64;
        let mut y1 = y1 as f64;

        let dx = (x1 - x0) as f64;
        let dy = (y1 - y0) as f64;

        let length = (dx * dx + dy * dy).sqrt();

        let step_x = dx / length;
        let step_y = dy / length;

        for i in 0..((length as i32) + 1) {
            let x = (x0 + ((i as f64) * step_x));
            let y = (y0 + ((i as f64) * step_y));
            self.aa_pixel(x as f64, y as f64, color);
        }
    }
}

impl RenderEngine for OrbGLRenderEngine {
    fn save(&mut self) {
        self.saved_states.push(self.state);
    }

    fn restore(&mut self) {
        if self.saved_states.len() > 0 {
            self.state = self.saved_states.pop().unwrap();
        }
    }

    fn fill(&mut self) {
        let color: Color;

        let mut edges: Vec<Edge>;

        color = self.state.fill_style;
        edges = self.path_builder.build();

        let mut min_y = 0.0;
        let mut max_y = 0.0;
        {
            let mut surface = self.surface.borrow_mut();
            min_y = surface.height() as f64;
        }

        for edge in &edges {
            let start_point = self.state.transform.apply_to_point(edge.start);
            let end_point = self.state.transform.apply_to_point(edge.end);

            if (start_point.y < min_y) {
                min_y = start_point.y;
            }

            if (end_point.y < min_y) {
                min_y = end_point.y;
            }

            if (start_point.y > max_y) {
                max_y = start_point.y;
            }

            if (end_point.y > max_y) {
                max_y = end_point.y;
            }
            //self.aa_line(start_point.x, start_point.y, end_point.x, end_point.y, color);
        }

        for y in min_y as i32..max_y as i32 {
            let mut lines = self.scanline(y as f64);

            let mut j: i32 = 0;
            while j < lines.len() as i32 {
                if j + 1 < lines.len() as i32 {
                    let x_start = lines[j as usize];
                    let x_stop = lines[(j + 1) as usize];

                    self.line((x_start + 1.0) as i32, y, (x_stop) as i32, y, color);

                    j = j + 2;
                } else {
                    j = lines.len() as i32;
                }
            }
        }

        for edge in &edges {
            let start_point = self.state.transform.apply_to_point(edge.start);
            let end_point = self.state.transform.apply_to_point(edge.end);
            self.aa_line(
                start_point.x,
                start_point.y,
                end_point.x,
                end_point.y,
                color,
            );
        }
    }

    fn stroke(&mut self) {
        let mut x: i32 = 0;
        let mut y: i32 = 0;

        let line_width: f64;
        let color: Color;
        let mut edges: Vec<Edge>;

        color = self.state.stroke_style;
        line_width = self.state.line_width;
        edges = self.path_builder.build();

        for edge in edges {
            match edge.edge_type {
                EdgeType::Visible => {
                    let start_point = self.state.transform.apply_to_point(edge.start);
                    let end_point = self.state.transform.apply_to_point(edge.end);

                    //ToDo: line width
                    let mut new_color = Color::rgba(color.r(), color.g(), color.b(), color.a());
                    if line_width < 1.0 {
                        let new_r = color.r() as f64 * line_width;
                        let new_g = color.g() as f64 * line_width;
                        let new_b = color.b() as f64 * line_width;
                        let new_a = color.a() as f64 * line_width;

                        new_color = Color::rgba(new_r as u8, new_g as u8, new_b as u8, new_a as u8);
                    }

                    self.aa_line(
                        start_point.x,
                        start_point.y,
                        end_point.x,
                        end_point.y,
                        new_color,
                    );
                }
                _ => {}
            }
        }
    }

    fn begin_path(&mut self) {
        self.path_builder = PathBuilder::new();
    }

    fn arc(&mut self, x: f64, y: f64, radius: f64, start_segment: f64, end_segment: f64) {
        let path_builder = &mut self.path_builder;
        path_builder.arc(x, y, radius, start_segment, end_segment);
    }

    fn close_path(&mut self) {
        let path_builder = &mut self.path_builder;
        path_builder.close_path();
    }

    fn move_to(&mut self, x: f64, y: f64) {
        let p = Point::new(x as f64, y as f64);
        let path_builder = &mut self.path_builder;
        path_builder.move_to(x, y);
    }

    fn line_to(&mut self, x: f64, y: f64) {
        let p = Point::new(x as f64, y as f64);
        let path_builder = &mut self.path_builder;
        path_builder.line_to(x, y);
    }

    fn quadratic_curve_to(&mut self, cpx: f64, cpy: f64, x: f64, y: f64) {
        let path_builder = &mut self.path_builder;
        path_builder.quadratic_curve_to(cpx, cpy, x, y);
    }

    fn bezier_curve_to(&mut self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
        let path_builder = &mut self.path_builder;
        path_builder.bezier_curve_to(cp1x, cp1y, cp2x, cp2y, x, y);
    }

    fn rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        let path_builder = &mut self.path_builder;
        path_builder.move_to(x, y);
        path_builder.line_to((x + width), y);
        path_builder.line_to((x + width), (y + height));
        path_builder.line_to(x, (y + height));
        path_builder.line_to(x, y);
    }

    fn fill_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        let tmp: PathBuilder = self.path_builder.clone();
        self.path_builder = PathBuilder::new();
        self.rect(x, y, width, height);
        self.fill();
        self.path_builder = tmp;
    }

    fn stroke_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        let tmp: PathBuilder = self.path_builder.clone();
        self.path_builder = PathBuilder::new();
        self.rect(x, y, width, height);
        self.stroke();
        self.path_builder = tmp;
    }

    fn clear_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        let tmp: PathBuilder = self.path_builder.clone();
        self.save();
        self.state.override_color = true;
        self.set_fill_style(Color::rgba(0, 0, 0, 0));
        self.path_builder = PathBuilder::new();
        self.rect(x, y, width, height);
        self.fill();
        self.restore();
        self.path_builder = tmp;
    }

    fn scale(&mut self, sx: f64, sy: f64) {
        self.state.transform.scale(sx, sy);
    }

    fn rotate(&mut self, angle: f64) {
        self.state.transform.rotate(angle);
    }

    fn translate(&mut self, tx: f64, ty: f64) {
        self.state.transform.translate(tx, ty);
    }

    fn transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        self.state.transform.transform(a, b, c, d, e, f);
    }

    fn set_transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        self.state.transform.set_transform(a, b, c, d, e, f);
    }

    fn set_fill_style(&mut self, color: Color) {
        self.state.fill_style = color;
    }

    fn set_stroke_style(&mut self, color: Color) {
        self.state.stroke_style = color;
    }

    fn set_line_width(&mut self, line_width: f64) {
        self.state.line_width = line_width;
    }
}
