use std::time::Instant;

use glium::glutin::api::egl::display;


pub struct FpsHandler {
    pub previous_frame: Instant,
}

impl FpsHandler {
    pub fn from_instant(start: Instant) -> Self {
        Self {previous_frame: start}
    }

    pub fn display_fps(&mut self) {
        println!("fps = {:.0}", 1.0 / (Instant::now() - self.previous_frame).as_secs_f32());
        self.previous_frame = Instant::now();
    }
}