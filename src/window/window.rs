extern crate winit;
extern crate glium;

use glium::{glutin::surface::WindowSurface, Display};
use winit::{dpi::LogicalPosition, event_loop::EventLoop, window::Window};

use crate::Data;

pub struct WindowHandler {
    pub window: Window,
    pub display: Display<WindowSurface>
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum WindowError {
    CursorGrabError
}

impl WindowHandler {
    pub fn new(window_name: &str, window_extent: (u32, u32), event_loop: &EventLoop<()>) -> Self{

        let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
                .with_title(window_name)
                .with_inner_size(window_extent.0, window_extent.1)
                .build(&event_loop);

        Self {
            window,
            display,
        }
    }

    pub fn unlock_cursor(&self, data: &mut Data) {
        self.window.set_cursor_grab(winit::window::CursorGrabMode::None).unwrap();
        self.window.set_cursor_visible(true);
        data.window_active = false;
    }

    pub fn lock_cursor(&self, data: &mut Data) {
        self.window
                .set_cursor_grab(winit::window::CursorGrabMode::Confined)
                .or_else(|_e| self.window.set_cursor_grab(winit::window::CursorGrabMode::Locked))
                .unwrap();
        self.window.set_cursor_visible(false);
        data.window_active = true;
        self.center_cursor(data);
    }

    pub fn center_cursor(&self, data: &mut Data) {
        self.window.set_cursor_position(LogicalPosition::new(data.window_extent.0 / 2.0, data.window_extent.1 / 2.0)).unwrap();
    }
}