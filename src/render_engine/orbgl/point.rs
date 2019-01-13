#[derive(Copy, Clone)]
pub enum PointType {
    Visible,
    Hidden,
}

#[derive(Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub point_type: PointType,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point {
            x: x,
            y: y,
            point_type: PointType::Visible,
        }
    }

    pub fn new_with_type(x: f64, y: f64, point_type: PointType) -> Self {
        Point {
            x: x,
            y: y,
            point_type: point_type,
        }
    }

    pub fn abs2(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2)
    }

    pub fn abs(&self) -> f64 {
        self.abs2().sqrt()
    }

    pub fn arg(&self) -> f64 {
        self.y.atan2(self.x)
    }

    pub fn vector(a: &Point, b: &Point) -> Point {
        Point { x: b.x - a.x, y: b.y - a.y, point_type: PointType::Visible }
    }

    pub fn cross_product(a: &Point, b: &Point) -> f64 {
        a.x * b.y - a.y * b.x
    }
}
