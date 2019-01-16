#![crate_name = "orbgl"]
#![crate_type = "lib"]

extern crate core;

#[cfg(not(target_arch = "wasm32"))]
extern crate orbclient;

#[cfg(not(target_arch = "wasm32"))]
extern crate orbimage;

#[cfg(not(target_arch = "wasm32"))]
extern crate rust_cairo;

#[cfg(target_arch = "wasm32")]
extern crate stdweb;

pub use crate::api::Color;

pub use api::*;
pub mod api;

pub use render_engine::*;
pub mod render_engine;

#[cfg(not(target_arch = "wasm32"))]
pub use surface::*;

#[cfg(not(target_arch = "wasm32"))]
pub mod surface;

/*
/// Canvas components
pub use point::Point;
pub use edge::Edge;
pub use pathbuilder::PathBuilder;
pub use matrix::Matrix;
pub use canvas::Canvas;
pub use canvaspaintstate::CanvasPaintState;


#[path = "canvas/point.rs"]
pub mod point;
#[path = "canvas/edge.rs"]
pub mod edge;
#[path = "canvas/pathbuilder.rs"]
pub mod pathbuilder;
#[path = "canvas/matrix.rs"]
pub mod matrix;
#[path = "canvas/canvas.rs"]
pub mod canvas;
#[path = "canvas/canvaspaintstate.rs"]
pub mod canvaspaintstate;
*/
