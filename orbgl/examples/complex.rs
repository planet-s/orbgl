#![allow(dead_code)]
use std::{fs::File, io::Read};
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
    //let surface = FramebufferSurface::new(800, 600, window.data_mut().as_mut_ptr() as *mut u8);
    let render_engine = OrbGLRenderEngine::new(surface.clone()); //
    let render_engine_cairo = CairoRenderEngine::new(surface.clone());
    let mut canvas = Canvas::new(render_engine_cairo.clone());

    //Transform the canvas
    canvas.transform(0.5, 0.0, 0.0, 0.5, 0.0, 0.0);

    //read a textfile with commands
    let file_path = "assets/tiger.txt";

    let mut file = File::open(&file_path).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    //read line by line and call the canvas functions
    for line in buffer.lines() {
        let tokens: Vec<&str> = line.split(" ").collect();

        if tokens.len() == 0 {
            continue;
        } else if tokens[0] == "save" {
            canvas.save();
        } else if tokens[0] == "restore" {
            canvas.restore();
        } else if tokens[0] == "fill" {
            canvas.fill();
        } else if tokens[0] == "stroke" {
            canvas.stroke();
        } else if tokens[0] == "begin_path" {
            canvas.begin_path();
        } else if tokens[0] == "close_path" {
            canvas.close_path();
        } else if tokens[0] == "move_to" {
            let x: f64 = tokens[1].parse().unwrap();
            let y: f64 = tokens[2].parse().unwrap();
            canvas.move_to(x, y);
        } else if tokens[0] == "line_to" {
            let x: f64 = tokens[1].parse().unwrap();
            let y: f64 = tokens[2].parse().unwrap();
            canvas.line_to(x, y);
        } else if tokens[0] == "bezier_curve_to" {
            let a: f64 = tokens[1].parse().unwrap();
            let b: f64 = tokens[2].parse().unwrap();
            let c: f64 = tokens[3].parse().unwrap();
            let d: f64 = tokens[4].parse().unwrap();
            let e: f64 = tokens[5].parse().unwrap();
            let f: f64 = tokens[6].parse().unwrap();
            canvas.bezier_curve_to(a, b, c, d, e, f);
        } else if tokens[0] == "transform" {
            let a: f64 = tokens[1].parse().unwrap();
            let b: f64 = tokens[2].parse().unwrap();
            let c: f64 = tokens[3].parse().unwrap();
            let d: f64 = tokens[4].parse().unwrap();
            let e: f64 = tokens[5].parse().unwrap();
            let f: f64 = tokens[6].parse().unwrap();
            canvas.transform(a, b, c, d, e, f);
        } else if tokens[0] == "set_fill_style" {
            let r: u8 = tokens[1].parse().unwrap();
            let g: u8 = tokens[2].parse().unwrap();
            let b: u8 = tokens[3].parse().unwrap();
            let a: u8 = tokens[4].parse().unwrap();
            canvas.set_fill_style(Color::rgba(r, g, b, a));
        } else if tokens[0] == "set_stroke_style" {
            let r: u8 = tokens[1].parse().unwrap();
            let g: u8 = tokens[2].parse().unwrap();
            let b: u8 = tokens[3].parse().unwrap();
            let a: u8 = tokens[4].parse().unwrap();
            canvas.set_stroke_style(Color::rgba(r, g, b, a));
        } else if tokens[0] == "set_line_width" {
            let mut line_width: f64 = tokens[1].parse().unwrap();
            canvas.set_line_width(line_width);
        }
    }

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
