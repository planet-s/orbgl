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
    //let surface = ImageSurface::new(800, 600);
    let surface = FramebufferSurface::new(800, 600, window.data_mut().as_mut_ptr() as *mut u8);
    let render_engine = CairoRenderEngine::new(surface.clone()); //OrbGLRenderEngine::new(surface.clone()); //CairoRenderEngine::new(surface.clone());
    let mut canvas = Canvas::new(render_engine.clone());

    //Transform the canvas
    canvas.transform(2.83, -2.83, 2.83, 2.83, 150.0, 300.0);

    //Set canvas fill style
    canvas.set_fill_style(Color::rgba(0, 0, 0, 255));

    canvas.begin_path();
    canvas.move_to(48.355, 17.922);
    canvas.bezier_curve_to(52.06, 20.245, 54.658, 24.176, 55.131, 28.739);
    canvas.bezier_curve_to(56.642, 29.445, 58.319, 29.851, 60.097, 29.851);
    canvas.bezier_curve_to(66.588, 29.851, 71.849, 24.59, 71.849, 18.1);
    canvas.bezier_curve_to(71.849, 11.609, 66.588, 6.348, 60.097, 6.348);
    canvas.bezier_curve_to(53.668, 6.35, 48.453, 11.517, 48.355, 17.922);
    canvas.close_path();
    canvas.move_to(40.656, 41.984);
    canvas.bezier_curve_to(47.147, 41.984, 52.408, 36.722, 52.408, 30.232);
    canvas.bezier_curve_to(52.408, 23.742, 47.146, 18.481, 40.656, 18.481);
    canvas.bezier_curve_to(34.166, 18.481, 28.902, 23.743, 28.902, 30.233);
    canvas.bezier_curve_to(28.902, 36.723, 34.166, 41.984, 40.656, 41.984);
    canvas.close_path();
    canvas.move_to(45.641, 42.785);
    canvas.line_to(35.669, 42.785);
    canvas.bezier_curve_to(27.372, 42.785, 20.622, 49.536, 20.622, 57.833);
    canvas.line_to(20.622, 70.028);
    canvas.line_to(20.653, 70.219);
    canvas.line_to(21.493, 70.482);
    canvas.bezier_curve_to(29.411, 72.956, 36.290, 73.781, 41.952, 73.781);
    canvas.bezier_curve_to(53.011, 73.781, 59.421, 70.628, 59.816, 70.427);
    canvas.line_to(60.601, 70.03);
    canvas.line_to(60.685, 70.03);
    canvas.line_to(60.685, 57.833);
    canvas.bezier_curve_to(60.688, 49.536, 53.938, 42.785, 45.641, 42.785);
    canvas.close_path();
    canvas.move_to(65.084, 30.653);
    canvas.line_to(55.189, 30.653);
    canvas.bezier_curve_to(55.082, 34.612, 53.392, 38.177, 50.719, 40.741);
    canvas.bezier_curve_to(58.094, 42.934, 63.49, 49.773, 63.49, 57.851);
    canvas.line_to(63.49, 61.609);
    canvas.bezier_curve_to(73.26, 61.251, 78.89, 58.482, 79.261, 58.296);
    canvas.line_to(80.046, 57.898);
    canvas.line_to(80.13, 57.898);
    canvas.line_to(80.13, 45.699);
    canvas.bezier_curve_to(80.13, 37.403, 73.38, 30.653, 65.084, 30.653);
    canvas.close_path();
    canvas.move_to(20.035, 29.853);
    canvas.bezier_curve_to(22.334, 29.853, 24.473, 29.182, 26.285, 28.039);
    canvas.bezier_curve_to(26.861, 24.282, 28.875, 20.999, 31.752, 18.763);
    canvas.bezier_curve_to(31.764, 18.543, 31.785, 18.325, 31.785, 18.103);
    canvas.bezier_curve_to(31.785, 11.612, 26.523, 6.351, 20.035, 6.351);
    canvas.bezier_curve_to(13.543, 6.351, 8.283, 11.612, 8.283, 18.103);
    canvas.bezier_curve_to(8.283, 24.591, 13.543, 29.853, 20.035, 29.853);
    canvas.close_path();
    canvas.move_to(30.589, 40.741);
    canvas.bezier_curve_to(27.929, 38.19, 26.245, 34.644, 26.122, 30.709);
    canvas.bezier_curve_to(25.755, 30.682, 25.392, 30.653, 25.018, 30.653);
    canvas.line_to(15.047, 30.653);
    canvas.bezier_curve_to(6.75, 30.653, 0.0, 37.403, 0.0, 45.699);
    canvas.line_to(0.0, 57.896);
    canvas.line_to(0.031, 58.084);
    canvas.line_to(0.871, 58.349);
    canvas.bezier_curve_to(7.223, 60.332, 12.892, 61.246, 17.816, 61.534);
    canvas.line_to(17.816, 57.851);
    canvas.bezier_curve_to(17.818, 49.773, 23.212, 42.936, 30.589, 40.74);

    //Fill the polygon and draw a stroke
    canvas.fill();

    canvas.set_transform(1.0,0.0,0.0,1.0,0.0,0.0);
    canvas.begin_path();


    canvas.begin_path();
    canvas.set_fill_style(Color::rgba(255, 0, 0, 255));
    canvas.arc(200.0, 200.0, 50.0,1.0 * std::f64::consts::PI,  2.0 * std::f64::consts::PI);
    canvas.fill();

    canvas.begin_path();
    canvas.set_fill_style(Color::rgba(0, 0, 255, 255));
    canvas.arc(200.0, 200.0, 50.0,0.0 * std::f64::consts::PI,  1.0 * std::f64::consts::PI);
    canvas.fill();

    canvas.begin_path();
    canvas.set_stroke_style(Color::rgba(0, 0, 0, 255));
    canvas.arc(200.0, 200.0, 50.0,0.0 * std::f64::consts::PI,  2.0 * std::f64::consts::PI);
    canvas.stroke();

    let mut image = Image::from_path("assets/test.png").unwrap();
    //canvas.draw_image(&mut image, 0.0, 0.0);
    canvas.draw_image_with_size(&mut image, 0.0, 0.0, 256.0, 256.0);
    //canvas.draw_image_with_clip_and_size(&mut image,50.0, 0.0, 100.0, 100.0, 10.0, 10.0, 120.0, 120.0);
    //window.image_fast(0, 0, 800, 600, surface.borrow_mut().get_image_data());



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
