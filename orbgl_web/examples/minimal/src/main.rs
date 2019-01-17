use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{
    document,
    window,
    CanvasRenderingContext2d
};

use stdweb::web::html_element::CanvasElement;

use orbgl_web::prelude::*;

fn main() {
    stdweb::initialize();

    let w = 800;
    let h = 600;

    let canvas: CanvasElement = document().query_selector( "#canvas" ).unwrap().unwrap().try_into().unwrap();
    canvas.set_width(w);
    canvas.set_height(h);
   
    let surface = WebSurface::new(w, h, canvas.get_context().unwrap());
    let render_engine = WebRenderEngine::new(surface);
    let mut canvas = Canvas::new(render_engine.clone());
    canvas.set_fill_style(Color::rgba(0, 0, 0, 255));
    canvas.fill_rect(10.0, 10.0, 100.0, 75.0);

    stdweb::event_loop();
}
