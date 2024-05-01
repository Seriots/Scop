extern crate glium;
extern crate winit;

mod window;
mod teapot;
mod parser;
mod data;
mod graphics;


use crate::window::WindowHandler;
use parser::*;
use data::*;
use graphics::*;
use winit::keyboard::{KeyCode, PhysicalKey};

fn main() {
    
    let mut data = Data::new(); 
    let window_handler = WindowHandler::new("Test", (720, 720));

    let obj = Object::new(teapot::VERTICES.to_vec(), teapot::NORMALS.to_vec(), teapot::INDICES.to_vec());
    let drawing_object = Drawing::new();

    let mut fps_handler = FpsHandler::from_instant(data.start_time);

    window_handler.event_loop.run(move |ev, window_target| {
        match ev {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    window_target.exit();
                },
                winit::event::WindowEvent::RedrawRequested => {
                    drawing_object.draw(&window_handler.display, &obj, &data);
                    fps_handler.display_fps();
                },
                winit::event::WindowEvent::Resized(window_size) => {
                    window_handler.display.resize(window_size.into());
                }
                winit::event::WindowEvent::KeyboardInput{ event, .. } => {
                    match event.physical_key {
                        PhysicalKey::Code(KeyCode::KeyW) => data.camera.move_forward(),
                        PhysicalKey::Code(KeyCode::KeyS) => data.camera.move_backward(),
                        PhysicalKey::Code(KeyCode::KeyA) => data.camera.move_left(),
                        PhysicalKey::Code(KeyCode::KeyD) => data.camera.move_right(),
                        
                        _ =>  println!("Physical key = {:?}", event.physical_key),
                    }
                    println!("Physical key = {:?}", event.physical_key);                    
                },
                _ => (),
            },
            winit::event::Event::AboutToWait => {
                window_handler.window.request_redraw();
            },
            _ => (),
        }
    }).unwrap();
}
