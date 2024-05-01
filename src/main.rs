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

                    let movement = data.key_event_handler.get_movement_vector();
                    let rotation = data.key_event_handler.get_rotation_vector();
                    drawing_object.draw(&window_handler.display, &obj, &data);
                    let fps = fps_handler.display_fps(false);
                    data.camera.rotate_from_vector3(rotation, 1.0 / fps);
                    data.camera.move_from_vector3(movement, 1.0 / fps);
                },
                winit::event::WindowEvent::Resized(window_size) => {
                    window_handler.display.resize(window_size.into());
                }
                winit::event::WindowEvent::KeyboardInput{ event, .. } => {
                    match event.state {
                        winit::event::ElementState::Pressed => match event.physical_key {
                            PhysicalKey::Code(KeyCode::KeyW) => data.key_event_handler.start_event(window::Movement::Forward),
                            PhysicalKey::Code(KeyCode::KeyS) => data.key_event_handler.start_event(window::Movement::Backward),
                            PhysicalKey::Code(KeyCode::KeyA) => data.key_event_handler.start_event(window::Movement::Left),
                            PhysicalKey::Code(KeyCode::KeyD) => data.key_event_handler.start_event(window::Movement::Right),
                            PhysicalKey::Code(KeyCode::Space) => data.key_event_handler.start_event(window::Movement::Up),
                            PhysicalKey::Code(KeyCode::ShiftLeft) => data.key_event_handler.start_event(window::Movement::Down),
                            PhysicalKey::Code(KeyCode::ArrowLeft) => data.key_event_handler.start_event(window::Movement::RotateLeft),
                            PhysicalKey::Code(KeyCode::ArrowRight) => data.key_event_handler.start_event(window::Movement::RotateRight),
                            
                            
                            _ =>  println!("Physical key = {:?}", event.physical_key),
                        },
                        winit::event::ElementState::Released => match event.physical_key {
                            PhysicalKey::Code(KeyCode::KeyW) => data.key_event_handler.stop_event(window::Movement::Forward),
                            PhysicalKey::Code(KeyCode::KeyS) => data.key_event_handler.stop_event(window::Movement::Backward),
                            PhysicalKey::Code(KeyCode::KeyA) => data.key_event_handler.stop_event(window::Movement::Left),
                            PhysicalKey::Code(KeyCode::KeyD) => data.key_event_handler.stop_event(window::Movement::Right),
                            PhysicalKey::Code(KeyCode::Space) => data.key_event_handler.stop_event(window::Movement::Up),
                            PhysicalKey::Code(KeyCode::ShiftLeft) => data.key_event_handler.stop_event(window::Movement::Down),
                            PhysicalKey::Code(KeyCode::ArrowLeft) => data.key_event_handler.stop_event(window::Movement::RotateLeft),
                            PhysicalKey::Code(KeyCode::ArrowRight) => data.key_event_handler.stop_event(window::Movement::RotateRight),
                            
                            _ =>  println!("Physical key = {:?}", event.physical_key),
                        },
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

    println!("HEre");
}
