#![crate_name="orbgl"]
#![crate_type="lib"]
#![feature(alloc)]
#![feature(asm)]

extern crate alloc;
extern crate core;
extern crate orbclient;

pub use orbclient::Color;

/// Canvas components
pub use point::Point;
pub use edge::Edge;
pub use pathbuilder::PathBuilder;
pub use matrix::Matrix;
pub use canvas::Canvas;
pub use canvaspaintstate::CanvasPaintState;



#[path="canvas/point.rs"]
pub mod point;
#[path="canvas/edge.rs"]
pub mod edge;
#[path="canvas/pathbuilder.rs"]
pub mod pathbuilder;
#[path="canvas/matrix.rs"]
pub mod matrix;
#[path="canvas/canvas.rs"]
pub mod canvas;
#[path="canvas/canvaspaintstate.rs"]
pub mod canvaspaintstate;

