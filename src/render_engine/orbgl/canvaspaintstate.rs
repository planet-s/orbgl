use orbclient::Color;
use super::matrix::Matrix;

#[derive(Copy, Clone)]
pub struct CanvasPaintState {
    pub fill_style: Color,
    pub stroke_style: Color,
    pub line_width: f64,
    pub transform: Matrix,
    pub override_color: bool,
}

impl CanvasPaintState {
    pub fn new() -> Self {
        CanvasPaintState {
            fill_style: Color::rgba(0, 0, 0, 0),
            stroke_style: Color::rgba(0, 0, 0, 0),
            line_width: 1.0,
            transform: Matrix::new(),
            override_color: false,
        }
    }
}