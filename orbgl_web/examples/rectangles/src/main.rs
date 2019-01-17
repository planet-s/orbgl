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

    //Transform the canvas
    canvas.transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0);


    //Set canvas fill style
    canvas.set_fill_style(Color::rgba(0, 0, 0, 255));
    canvas.set_stroke_style(Color::rgba(0, 0, 0, 255));

    canvas.stroke_rect(100.0, 20.0, 100.0, 100.0);
    canvas.fill_rect(20.0, 100.0, 100.0, 100.0);


    canvas.set_fill_style(Color::rgba(255, 0, 0, 255));
    canvas.set_stroke_style(Color::rgba(0, 0, 0, 255));

    //canvas.begin_path();
    canvas.move_to(30.0,30.0);
    canvas.line_to(50.0,50.0);
    canvas.fill_rect(20.0, 20.0, 20.0, 20.0);
    canvas.line_to(100.0,55.0);
    //canvas.close_path();
    canvas.stroke();
    canvas.clear_rect(0.0,0.0,30.0,30.0);

    stdweb::event_loop();
}
