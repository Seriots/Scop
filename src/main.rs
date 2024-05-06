extern crate glium;
extern crate winit;

mod window;
mod teapot;
mod parser;
mod data;
mod graphics;
mod matrix;


use crate::window::WindowHandler;
use parser::*;
use data::*;
use graphics::*;
use winit::keyboard::{KeyCode, PhysicalKey};

fn main() {
    
    let event_loop = winit::event_loop::EventLoopBuilder::new()
    .build()
    .expect("event loop building");
    let window_handler = WindowHandler::new("Test", (720, 720), &event_loop);

    let mut data = Data::new((window_handler.window.inner_size().width as f32, window_handler.window.inner_size().height as f32));
    
    let obj = Object::new(teapot::VERTICES.to_vec(), teapot::NORMALS.to_vec(), teapot::INDICES.to_vec());
    let mut drawing_object = Drawing::new();
    drawing_object.compute_program(&window_handler.display, &String::from("src/shaders/shader.vert"), &String::from("src/shaders/shader.frag"));

    let mut fps_handler = FpsHandler::from_instant(data.start_time);

    event_loop.run(move |ev, window_target| {
        match ev {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    window_target.exit();
                },
                winit::event::WindowEvent::RedrawRequested => {

                    let movement = data.key_event_handler.get_movement_vector();
                    let rotation = data.key_event_handler.get_rotation_vector();
                    let fps = fps_handler.display_fps(false);
                    data.camera.rotate_from_vector3(rotation, 1.0 / fps * 200.0);
                    data.camera.move_from_vector3(movement, 1.0 / fps);
                    drawing_object.draw(&window_handler.display, &obj, &data, &fps_handler.delta_time);
                },
                winit::event::WindowEvent::Resized(window_size) => {
                    window_handler.display.resize(window_size.into());
                    data.window_extent = (window_size.width as f32, window_size.height as f32);
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
                            PhysicalKey::Code(KeyCode::ArrowUp) => data.key_event_handler.start_event(window::Movement::RotateUp),
                            PhysicalKey::Code(KeyCode::ArrowDown) => data.key_event_handler.start_event(window::Movement::RotateDown),
                            PhysicalKey::Code(KeyCode::KeyQ) => data.key_event_handler.start_event(window::Movement::RotateEast),
                            PhysicalKey::Code(KeyCode::KeyE) => data.key_event_handler.start_event(window::Movement::RotateWest),
                            PhysicalKey::Code(KeyCode::Escape) => window_handler.unlock_cursor(&mut data),
                            
                            _ =>  (),
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
                            PhysicalKey::Code(KeyCode::ArrowUp) => data.key_event_handler.stop_event(window::Movement::RotateUp),
                            PhysicalKey::Code(KeyCode::ArrowDown) => data.key_event_handler.stop_event(window::Movement::RotateDown),
                            PhysicalKey::Code(KeyCode::KeyQ) => data.key_event_handler.stop_event(window::Movement::RotateEast),
                            PhysicalKey::Code(KeyCode::KeyE) => data.key_event_handler.stop_event(window::Movement::RotateWest),
                            
                            _ =>  (),
                        },
                    }
                },
                winit::event::WindowEvent::CursorMoved {position, ..} =>  {
                    if data.window_active {
                        data.key_event_handler.mouse_moved(position, &data.window_extent, &mut data.camera);
                        window_handler.center_cursor(&mut data);
                    }
                },
                winit::event::WindowEvent::MouseInput { state, button, .. } => {
                    match state {
                        winit::event::ElementState::Pressed => match button {
                            winit::event::MouseButton::Left => {
                                window_handler.lock_cursor(&mut data);
                            },

                            _ => (),
                            },
                        _ => {},
                    }
                }
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
