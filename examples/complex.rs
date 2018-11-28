#![allow(dead_code)]
extern crate orbclient;
extern crate orbgl;

use std::fs::File;
use std::io::Read;

use orbclient::{Color, Window, Renderer, EventOption};
use orbgl::Canvas;


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
    let mut canvas = Canvas::new(800.0, 600.0);

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
            let x: f32 = tokens[1].parse().unwrap();
            let y: f32 = tokens[2].parse().unwrap();
            canvas.move_to(x, y);
        } else if tokens[0] == "line_to" {
            let x: f32 = tokens[1].parse().unwrap();
            let y: f32 = tokens[2].parse().unwrap();
            canvas.line_to(x, y);
        } else if tokens[0] == "bezier_curve_to" {
            let a: f32 = tokens[1].parse().unwrap();
            let b: f32 = tokens[2].parse().unwrap();
            let c: f32 = tokens[3].parse().unwrap();
            let d: f32 = tokens[4].parse().unwrap();
            let e: f32 = tokens[5].parse().unwrap();
            let f: f32 = tokens[6].parse().unwrap();
            canvas.bezier_curve_to(a, b, c, d, e, f);
        } else if tokens[0] == "transform" {
            let a: f32 = tokens[1].parse().unwrap();
            let b: f32 = tokens[2].parse().unwrap();
            let c: f32 = tokens[3].parse().unwrap();
            let d: f32 = tokens[4].parse().unwrap();
            let e: f32 = tokens[5].parse().unwrap();
            let f: f32 = tokens[6].parse().unwrap();
            canvas.transform(a, b, c, d, e, f);
        } else if tokens[0] == "set_fill_style" {
            let r: u8 = tokens[1].parse().unwrap();
            let g: u8 = tokens[2].parse().unwrap();
            let b: u8 = tokens[3].parse().unwrap();
            canvas.set_fill_style(Color::rgb(r, g, b));
        } else if tokens[0] == "set_stroke_style" {
            let r: u8 = tokens[1].parse().unwrap();
            let g: u8 = tokens[2].parse().unwrap();
            let b: u8 = tokens[3].parse().unwrap();
            canvas.set_stroke_style(Color::rgb(r, g, b));
        }
    }


    window.image_fast(0, 0, 800, 600, unsafe { &canvas.data });


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
