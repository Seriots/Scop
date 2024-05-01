extern crate winit;
extern crate glium;

use glium::{glutin::surface::WindowSurface, Display};
use winit::{event_loop::EventLoop, window::Window};

pub struct WindowHandler {
    pub event_loop: EventLoop<()>,
    pub window: Window,
    pub display: Display<WindowSurface>
}

impl WindowHandler {
    pub fn new(window_name: &str, window_extent: (u32, u32)) -> Self {
        let event_loop = winit::event_loop::EventLoopBuilder::new()
                .build()
                .expect("event loop building");

        let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
                .with_title(window_name)
                .with_inner_size(window_extent.0, window_extent.1)
                .build(&event_loop);
        Self {event_loop, window, display}
    }
}