#[derive(Copy, Clone)]
pub enum PointType {
    Visible,
    Hidden,
}

#[derive(Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub point_type: PointType,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Point {
            x: x,
            y: y,
            point_type: PointType::Visible,
        }
    }

    pub fn new_with_type(x: f32, y: f32, point_type: PointType) -> Self {
        Point {
            x: x,
            y: y,
            point_type: point_type,
        }
    }

    pub fn abs2(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2)
    }

    pub fn abs(&self) -> f32 {
        self.abs2().sqrt()
    }

    pub fn arg(&self) -> f32 {
        self.y.atan2(self.x)
    }

    pub fn vector(a: &Point, b: &Point) -> Point {
        Point { x: b.x - a.x, y: b.y - a.y, point_type: PointType::Visible }
    }

    pub fn cross_product(a: &Point, b: &Point) -> f32 {
        a.x * b.y - a.y * b.x
    }
}
