#![allow(dead_code)]
use orbclient::{Window, EventOption, Renderer};
use orbgl::prelude::*;
use std::time::Instant;

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
    let render_engine = CairoRenderEngine::new(surface.clone());
    let mut canvas = Canvas::new(render_engine.clone());

    let mut image = Image::from_path("assets/test.png").unwrap();
    canvas.draw_image_with_size(&mut image,  0.0, 0.0, 800.0, 600.0);
    canvas.draw_image_with_clip_and_size(&mut image, 5.0,5.0, 50.0, 50.0, 0.0, 0.0, 256.0, 256.0);

    window.sync();

    let mut deg = 0.0;
    let mut start = Instant::now();
    'event: loop {

        let end = Instant::now();
        let delta = end.duration_since(start);
        let delta_ms = delta.as_secs() as f32 * 1000_f32 + (delta.subsec_nanos() as f32)/1000000 as f32;
        start = end;

        let speed = delta_ms / 1000.0;

        deg = if(deg >= 1.0) { 0.0 } else { deg + 0.5 * speed as f64 };

        canvas.set_transform(1.0,0.0,0.0,1.0,0.0,0.0);
        canvas.draw_image_with_size(&mut image,  0.0, 0.0, 800.0, 600.0);


        canvas.set_transform(1.0,0.0,0.0,1.0,400.0,300.0);
        canvas.rotate(-deg * std::f64::consts::PI);
        canvas.translate(-256.0, -256.0);
        canvas.draw_image_with_clip_and_size(&mut image, 0.0,0.0, 160.0, 160.0, 0.0, 0.0, 512.0, 512.0);


        canvas.set_transform(1.0,0.0,0.0,1.0,400.0,300.0);
        canvas.rotate(deg * std::f64::consts::PI);

        canvas.translate(-128.0, -128.0);
        canvas.draw_image_with_clip_and_size(&mut image, 0.0,0.0, 80.0, 80.0, 0.0, 0.0, 256.0, 256.0);



        window.sync();


        for orbital_event in window.events() {
            match orbital_event.to_option() {
                EventOption::Quit(_quit_event) => break 'event,
                _ => (),
            };
        }
    }
}
