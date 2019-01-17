use std::{
    rc::Rc,
    cell::RefCell,
};

use orbgl_api::{Color, Surface};

pub struct FramebufferSurface {
    pub width: u32,
    pub height: u32,
    pub framebuffer: *mut u8,
}

impl FramebufferSurface {
    pub fn new(width: u32, height: u32, framebuffer: *mut u8) -> Rc<RefCell<FramebufferSurface>>  {
        Rc::new(RefCell::new(Self {
           width,
            height,
            framebuffer: framebuffer,
        }))
    }
}

impl Surface for FramebufferSurface {
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn data_mut(&mut self) -> &mut [Color] {
        unsafe {
            let len = (self.width * self.height) as usize;
            let ptr = self.framebuffer;
            std::slice::from_raw_parts_mut(ptr as *mut Color, len)
        }
    }
}