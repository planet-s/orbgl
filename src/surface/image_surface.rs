use super::surface::*;
use orbclient::Color;
use std::rc::Rc;
use std::cell::RefCell;

pub struct ImageSurface {
    pub width: u32,
    pub height: u32,
    pub data: Vec<Color>,
}


impl ImageSurface {
    pub fn new(width: u32, height: u32) -> Rc<RefCell<ImageSurface>> {
        let size: usize = (width * height) as usize;
        Rc::new(RefCell::new(
        Self {
            width: width,
            height: height,
            data: vec![Color::rgba(0, 0, 0, 0); size],
        }
        ))
    }

    pub fn get_image_data(&self) -> &Vec<Color>{
        &self.data
    }
}

impl Surface for ImageSurface {
    fn width(&self) -> u32 {
        self.width
    }

    /// Get height
    fn height(&self) -> u32 {
        self.height
    }

    fn data_mut(&mut self) -> &mut [Color]{
        &mut self.data
    }
}