use orbclient::Color;
use point::Point;
use pathbuilder::PathBuilder;
use edge::Edge;
use edge::EdgeType;
use canvaspaintstate::CanvasPaintState;


#[repr(packed)]
#[allow(unused)]
pub struct Canvas {
    pub width: f32,
    pub height: f32,
    pub data: Vec<Color>,
    path_builder: PathBuilder,
    state: CanvasPaintState,
    saved_states: Vec<CanvasPaintState>,
}

impl Canvas {
    pub fn new(width: f32, height: f32) -> Self {
        let size: u64 = (width * height) as u64;


        Canvas {
            width: width,
            height: height,
            data: vec![Color::rgba(0, 0, 0, 0); size as usize],
            path_builder: PathBuilder::new(),
            state: CanvasPaintState::new(),
            saved_states: vec![],
        }
    }

    fn pixel(&mut self, x: i32, y: i32, color: Color) {
        let replace = false;

        let w = self.width as i32;
        let h = self.height as i32;
        let data = unsafe { &mut self.data };

        if x >= 0 && y >= 0 && x < w as i32 && y < h as i32 {
            let new = color.data;
            let alpha = (new >> 24) & 0xFF;
            let old = unsafe { &mut data[y as usize * w as usize + x as usize].data };

            if alpha >= 255 || replace {
                *old = new;
            } else if alpha > 0 {
                let n_alpha = 255 - alpha;
                let rb = ((n_alpha * (*old & 0x00FF00FF)) + (alpha * (new & 0x00FF00FF))) >> 8;
                let ag = (n_alpha * ((*old & 0xFF00FF00) >> 8)) + (alpha * (0x01000000 | ((new & 0x0000FF00) >> 8)));

                *old = (rb & 0x00FF00FF) | (ag & 0xFF00FF00);
            }
        }
    }

    pub fn line(&mut self, argx1: i32, argy1: i32, argx2: i32, argy2: i32, color: Color) {
        let mut x = argx1;
        let mut y = argy1;

        let dx = if argx1 > argx2 { argx1 - argx2 } else { argx2 - argx1 };
        let dy = if argy1 > argy2 { argy1 - argy2 } else { argy2 - argy1 };

        let sx = if argx1 < argx2 { 1 } else { -1 };
        let sy = if argy1 < argy2 { 1 } else { -1 };

        let mut err = if dx > dy { dx } else { -dy } / 2;
        let mut err_tolerance;

        loop {
            self.pixel(x, y, color);
            //self.wu_circle(x,y,1,color);

            if x == argx2 && y == argy2 { break; };

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
    fn aa_pixel(&mut self, x: f32, y: f32, color: Color) {
        let r = color.r();
        let g = color.g();
        let b = color.b();
        let a = color.a();

        let int_x = x as i32;
        let int_y = y as i32;
        let frac_x = x - int_x as f32;
        let frac_y = y - int_y as f32;

        let _a = (1.0 - frac_x) * (1.0 - frac_y);
        let _b = frac_x * (1.0 - frac_y);
        let _c = (1.0 - frac_x) * frac_y;
        let _d = frac_x * frac_y;

        self.pixel(int_x, int_y, Color::rgba(r, g, b, (a as f32 * _a) as u8));

        self.pixel(int_x + 1, int_y, Color::rgba(r, g, b, (a as f32 * _b) as u8));
        self.pixel(int_x, int_y + 1, Color::rgba(r, g, b, (a as f32 * _c) as u8));
        self.pixel(int_x + 1, int_y + 1, Color::rgba(r, g, b, (a as f32 * _d) as u8));
    }


    /// antialiased line
    #[allow(unused)]
    fn aa_line(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, color: Color) {
        let mut x0 = x0 as f32;
        let mut y0 = y0 as f32;
        let mut x1 = x1 as f32;
        let mut y1 = y1 as f32;

        let dx = (x1 - x0) as f32;
        let dy = (y1 - y0) as f32;

        let length = (dx * dx + dy * dy).sqrt();

        let step_x = dx / length;
        let step_y = dy / length;

        for i in 0..((length as i32) + 1) {
            let x = (x0 + ((i as f32) * step_x));
            let y = (y0 + ((i as f32) * step_y));
            self.aa_pixel(x as f32, y as f32, color);
        }
    }
}


/// Common
#[allow(unused)]
impl Canvas {
    pub fn save(&mut self) {
        unsafe {
            self.saved_states.push(self.state);
        }
    }

    pub fn restore(&mut self) {
        unsafe {
            if self.saved_states.len() > 0 {
                self.state = self.saved_states.pop().unwrap();
            }
        }
    }
}

//Paths
#[allow(unused)]
impl Canvas {
    pub fn scanline(&mut self, y: f32) -> Vec<f32> {
        let mut cross_points: Vec<f32> = Vec::new();

        let mut edges: Vec<Edge>;
        unsafe {
            edges = self.path_builder.build();
        }


        for edge in edges {
            let start_point = unsafe { self.state.transform.apply_to_point(edge.start) };
            let end_point = unsafe { self.state.transform.apply_to_point(edge.end) };
            let t: f32 = (((end_point.x - start_point.x) * (y as f32 - start_point.y)) / (end_point.y - start_point.y)) + start_point.x;
            if (start_point.y > y as f32) != (end_point.y > y as f32) {
                cross_points.push(t as f32);
            }
        }

        cross_points.sort_by(|a, b| a.partial_cmp(&b).unwrap());
        cross_points
    }


    pub fn fill(&mut self) {
        let color: Color;

        let mut edges: Vec<Edge>;
        unsafe {
            color = self.state.fill_style;
            edges = self.path_builder.build();
        }

        for y in 0..self.height as i32 {
            let mut lines = self.scanline(y as f32);


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

        for edge in edges {
            let start_point = unsafe { self.state.transform.apply_to_point(edge.start) };
            let end_point = unsafe { self.state.transform.apply_to_point(edge.end) };
            self.aa_line(start_point.x, start_point.y, end_point.x, end_point.y, color);
        }
    }

    pub fn stroke(&mut self) {
        let mut x: i32 = 0;
        let mut y: i32 = 0;

        let line_width: f32;
        let color: Color;
        let mut edges: Vec<Edge>;
        unsafe {
            color = self.state.stroke_style;
            line_width = self.state.line_width;
            edges = self.path_builder.build();
        }

        for edge in edges {
            match edge.edge_type {
                EdgeType::Visible => {
                    let start_point = unsafe { self.state.transform.apply_to_point(edge.start) };
                    let end_point = unsafe { self.state.transform.apply_to_point(edge.end) };

                    if line_width == 1.0 {
                        self.aa_line(start_point.x, start_point.y, end_point.x, end_point.y, color);
                    }
                }
                _ => {}
            }
        }
    }

    ///
    pub fn begin_path(&mut self) {
        self.path_builder = PathBuilder::new();
    }

    ///
    pub fn close_path(&mut self) {
        let path_builder = unsafe { &mut self.path_builder };
        path_builder.close_path();
    }

    /// move to position
    pub fn move_to(&mut self, x: f32, y: f32) {
        let p = Point::new(x as f32, y as f32);
        let path_builder = unsafe { &mut self.path_builder };
        path_builder.move_to(x, y);
    }

    /// create a line between the last and new point
    pub fn line_to(&mut self, x: f32, y: f32) {
        let p = Point::new(x as f32, y as f32);
        let path_builder = unsafe { &mut self.path_builder };
        path_builder.line_to(x, y);
    }

    /// quadratic bezier curve
    pub fn quadratic_curve_to(&mut self, cpx: f32, cpy: f32, x: f32, y: f32) {
        let path_builder = unsafe { &mut self.path_builder };
        path_builder.quadratic_curve_to(cpx, cpy, x, y);
    }

    /// cubic bezier curve
    pub fn bezier_curve_to(&mut self, cp1x: f32, cp1y: f32, cp2x: f32, cp2y: f32, x: f32, y: f32) {
        let path_builder = unsafe { &mut self.path_builder };
        path_builder.bezier_curve_to(cp1x, cp1y, cp2x, cp2y, x, y);
    }

    pub fn rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        let path_builder = unsafe { &mut self.path_builder };
        path_builder.move_to(x, y);
        path_builder.line_to((x + width), y);
        path_builder.line_to((x + width), (y + height));
        path_builder.line_to(x, (y + height));
        path_builder.line_to(x, y);
    }
}


/// Transformations
#[allow(unused)]
impl Canvas {
    /// Scales the current drawing bigger or smaller
    pub fn scale(&mut self, sx: f32, sy: f32) {
        unsafe {
            self.state.transform.scale(sx, sy);
        }
    }

    /// Rotates the current drawing
    pub fn rotate(&mut self, angle: f32) {
        unsafe {
            self.state.transform.rotate(angle);
        }
    }

    /// Remaps the (0,0) position on the canvas
    pub fn translate(&mut self, tx: f32, ty: f32) {
        unsafe {
            self.state.transform.translate(tx, ty);
        }
    }

    /// Replaces the current transformation matrix for the drawing
    pub fn transform(&mut self, a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) {
        unsafe {
            self.state.transform.transform(a, b, c, d, e, f);
        }
    }

    /// Resets the current transform to the identity matrix. Then runs transform()
    pub fn set_transform(&mut self, a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) {
        unsafe {
            self.state.transform.set_transform(a, b, c, d, e, f);
        }
    }
}


/// Transformations
#[allow(unused)]
impl Canvas {
    pub fn set_fill_style(&mut self, color: Color) {
        self.state.fill_style = color;
    }

    pub fn set_stroke_style(&mut self, color: Color) {
        self.state.stroke_style = color;
    }

    pub fn set_line_width(&mut self, line_width: f32) {
        self.state.line_width = line_width;
    }
}
