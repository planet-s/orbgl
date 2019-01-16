use super::point::Point;

#[derive(Copy, Clone)]
pub enum EdgeType {
    Visible,
    Hidden,
}

#[derive(Copy, Clone)]
pub struct Edge {
    pub start: Point,
    pub end: Point,
    pub edge_type: EdgeType,
}

impl Edge {
    pub fn new(start: Point, end: Point) -> Self {
        Edge {
            start: start,
            end: end,
            edge_type: EdgeType::Visible,
        }
    }

    pub fn new_with_type(start: Point, end: Point, edge_type: EdgeType) -> Self {
        Edge {
            start: start,
            end: end,
            edge_type: edge_type,
        }
    }
}
