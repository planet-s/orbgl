use super::{
    edge::{Edge, EdgeType},
    point::{Point, PointType},
};

/// graphic path with similar functions like html canvas
#[derive(Clone)]
pub struct PathBuilder {
    point_counter: i32,
    pub points: Vec<Point>,
    pub edges: Vec<Edge>,
}

impl PathBuilder {
    pub fn new() -> Self {
        PathBuilder {
            point_counter: 0,
            points: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn build(&mut self) -> Vec<Edge> {
        let mut edges: Vec<Edge> = Vec::new();

        if self.points.len() == 0 {
            return edges;
        }

        let mut last_moved_point_index = 0;

        for i in 1..self.points.len() {
            match self.points[i].point_type {
                PointType::Hidden => { last_moved_point_index = i; }
                PointType::Visible => {
                    edges.push(Edge::new(self.points[i - 1].clone(), self.points[i].clone()));

                    if i == self.points.len() - 1 {
                        edges.push(Edge::new_with_type(self.points[i].clone(), self.points[last_moved_point_index].clone(), EdgeType::Hidden));
                    }

                    if i < self.points.len() - 1 {
                        match self.points[i + 1].point_type {
                            PointType::Hidden => { edges.push(Edge::new_with_type(self.points[i].clone(), self.points[last_moved_point_index].clone(), EdgeType::Hidden)); }
                            _ => {}
                        }
                    }
                }
            }
        }
        return edges;
    }

    pub fn arc(&mut self, x: f64, y: f64, radius: f64, start_segment: f64, end_segment: f64) {
        let mut t: f64 = 0.0;

        let start = start_segment;
        let mut end = end_segment;

        if start > 0.0 && end == 0.0 {
            end = 2.0 * std::f64::consts::PI;
        }

        if start != end {
            let segment_length = end - start;
            while t < 1.00 {
                let theta: f64 = start + (segment_length * t);
                let x_offset: f64 = radius * theta.cos();
                let y_offset: f64 = radius * theta.sin();
                self.points.push(Point::new(x + x_offset, y + y_offset));
                t += 1.0 / (segment_length * 10.0);
            }

            //last point
            let theta: f64 = start + (segment_length * 1.0);
            let x_offset: f64 = radius * theta.cos();
            let y_offset: f64 = radius * theta.sin();
            self.points.push(Point::new(x + x_offset, y + y_offset));
        }
    }

    /// move to position
    pub fn move_to(&mut self, x: f64, y: f64) {
        self.points.push(Point::new_with_type(x, y, PointType::Hidden));
    }

    /// create a line between the last and new point
    pub fn line_to(&mut self, x: f64, y: f64) {
        self.points.push(Point::new(x, y));
    }

    pub fn close_path(&mut self) {
        if self.points.len() == 0 {
            return;
        }

        let mut close_point: Point = self.points[0].clone();
        for point in &self.points {
            match point.point_type {
                PointType::Hidden => { close_point = point.clone() }
                _ => {}
            }
        }
        self.points.push(Point::new(close_point.x, close_point.y));
    }


    /// quadratic bezier curve
    pub fn quadratic_curve_to(&mut self, argx1: f64, argy1: f64, argx2: f64, argy2: f64) {
        let mut t: f64 = 0.0;
        let mut u: f64;
        let mut tt: f64;
        let mut uu: f64;
        let mut x: f64;
        let mut y: f64;

        let tmp_point = self.points[self.points.len() - 1];
        while t < 1.0 {
            u = 1.0 - t;
            uu = u * u;
            tt = t * t;

            x = tmp_point.x * uu;
            y = tmp_point.y * uu;

            x += 2.0 * u * t * argx1;
            y += 2.0 * u * t * argy1;

            x += tt * argx2;
            y += tt * argy2;

            t += 0.1;

            self.points.push(Point::new(x, y));
        }

        self.points.push(Point::new(argx2, argy2));
    }

    /// cubic bezier curve
    pub fn bezier_curve_to(&mut self, argx1: f64, argy1: f64, argx2: f64, argy2: f64, argx3: f64, argy3: f64) {
        let mut t: f64 = 0.0;
        let mut u: f64;
        let mut tt: f64;
        let mut uu: f64;
        let mut uuu: f64;
        let mut ttt: f64;
        let mut x: f64;
        let mut y: f64;

        let tmp_point = self.points[self.points.len() - 1];
        while t < 1.0 {
            u = 1.0 - t;
            tt = t * t;
            uu = u * u;
            uuu = uu * u;
            ttt = tt * t;

            x = tmp_point.x as f64 * uuu;
            y = tmp_point.y as f64 * uuu;

            x += 3.0 * uu * t * argx1 as f64;
            y += 3.0 * uu * t * argy1 as f64;

            x += 3.0 * u * tt * argx2 as f64;
            y += 3.0 * u * tt * argy2 as f64;

            x += ttt * argx3 as f64;
            y += ttt * argy3 as f64;

            t += 0.1;

            self.points.push(Point::new(x, y));
        }

        self.points.push(Point::new(argx3, argy3));
    }
}
