use super::point::Point;

#[derive(Copy, Clone)]
pub struct Matrix {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub e: f64,
    pub f: f64,
}

impl Matrix {
    pub fn new() -> Self {
        Matrix {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            e: 0.0,
            f: 0.0,
        }
    }

    pub fn scale(&mut self, sx: f64, sy: f64) {
        self.transform(sx, 0.0, 0.0, sy, 0.0, 0.0);
    }

    pub fn translate(&mut self, tx: f64, ty: f64) {
        self.transform(1.0, 0.0, 0.0, 1.0, tx, ty);
    }

    pub fn rotate(&mut self, angle: f64) {
        let cos = angle.cos();
        let sin = angle.sin();
        self.transform(cos, sin, -sin, cos, 0.0, 0.0);
    }

    pub fn set_transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        self.a = a;
        self.b = b;
        self.c = c;
        self.d = d;
        self.e = e;
        self.f = f;
    }


    pub fn transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        let org_a = self.a;
        let org_b = self.b;
        let org_c = self.c;
        let org_d = self.d;
        let org_e = self.e;
        let org_f = self.f;

        self.a = org_a * a + org_c * b;
        self.b = org_b * a + org_d * b;
        self.c = org_a * c + org_c * d;
        self.d = org_b * c + org_d * d;
        self.e = org_a * e + org_c * f + org_e;
        self.f = org_b * e + org_d * f + org_f;
    }

    pub fn apply_to_point(&mut self, point: Point) -> (Point) {
        Point::new(
            point.x * self.a + point.y * self.c + self.e,
            point.x * self.b + point.y * self.d + self.f,
        )
    }
}
