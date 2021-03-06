#![allow(dead_code)]
use orbclient::{Window, EventOption, Renderer};
use orbgl::prelude::*;

fn main() {
    let w = 800;
    let h = 600;
    let (width, height) = orbclient::get_display_size().unwrap();
    let mut window = Window::new_flags((width as i32) / 4,
                                       (height as i32) / 4,
                                       w,
                                       h,
                                       "OrbGL", &[orbclient::WindowFlag::Async]).unwrap();
    let (win_w, win_h) = (w, h);
    window.rect(0, 0, win_w, win_h, Color::rgb(255, 255, 255));

    //Init a new canvas
    let surface = ImageSurface::new(800, 600);
    let render_engine = CairoRenderEngine::new(surface.clone());
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
    // canvas.rect(0.0, 0.0, 30.0, 30.0);
    // canvas.clip();
    canvas.move_to(30.0,30.0);
    canvas.line_to(50.0,50.0);
    canvas.fill_rect(20.0, 20.0, 20.0, 20.0);
    canvas.line_to(100.0,55.0);
    //canvas.close_path();
    canvas.stroke();
    canvas.clear_rect(0.0,0.0,30.0,30.0);


    window.image_fast(0, 0, 800, 600, surface.borrow_mut().get_image_data());


    window.sync();


    'event: loop {
        for orbital_event in window.events() {
            match orbital_event.to_option() {
                EventOption::Quit(_quit_event) => break 'event,
                _ => (),
            };
        }
    }
}
