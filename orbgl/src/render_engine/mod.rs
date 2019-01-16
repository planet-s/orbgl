#[cfg(not(target_arch = "wasm32"))]
pub use self::orbgl::*;
#[cfg(not(target_arch = "wasm32"))]
mod orbgl;

#[cfg(not(target_arch = "wasm32"))]
pub use self::cairo::*;
#[cfg(not(target_arch = "wasm32"))]
mod cairo;

#[cfg(target_arch = "wasm32")]
pub use self::web::*;

#[cfg(target_arch = "wasm32")]
mod web;

pub use self::render_engine::*;
mod render_engine;
