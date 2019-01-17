use std::{cell::RefCell, rc::Rc};

use stdweb::web::CanvasRenderingContext2d;

use orbgl_api::{Color, Surface};

pub struct WebSurface {
    pub width: u32,
    pub height: u32,
    pub context: CanvasRenderingContext2d,
    pub data: Vec<Color>,
}

impl WebSurface {
    pub fn new(width: u32, height: u32, context: CanvasRenderingContext2d) -> WebSurface {
        let size = width * height;
        WebSurface {
            width,
            height,
            context,
            data: vec![Color::rgba(0, 0, 0, 0); size as usize],
        }
    }

    pub fn context(&mut self) -> &mut CanvasRenderingContext2d {
        &mut self.context
    }
}

impl Surface for WebSurface {
    fn width(&self) -> u32 {
        self.width
    }

    /// Get height
    fn height(&self) -> u32 {
        self.height
    }

    fn data_mut(&mut self) -> &mut [Color] {
        &mut self.data
    }
}
