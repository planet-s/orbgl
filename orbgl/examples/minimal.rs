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

    let surface = FramebufferSurface::new(800, 600, window.data_mut().as_mut_ptr() as *mut u8);
    let render_engine = OrbGLRenderEngine::new(surface.clone()); 
    let mut canvas = Canvas::new(render_engine.clone());

    canvas.set_fill_style(Color::rgba(0, 0, 0, 255));
    canvas.fill_rect(10.0, 10.0, 100.0, 75.0);
    
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
