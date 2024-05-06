use winit::dpi::PhysicalPosition;

use crate::{matrix::Vector, Camera};


#[derive(PartialEq)]
pub enum Movement {
	Forward,
	Backward,
	Left,
	Right,
	Up,
	Down,
	RotateLeft,
	RotateRight,
	RotateUp,
	RotateDown,
	RotateWest,
	RotateEast,
}

pub struct KeyEventHandler {
	pub movement: Vec<Movement>,
}

impl KeyEventHandler {
	pub fn new() -> Self {
		Self {movement: Vec::new()}
	}

	pub fn start_event(&mut self, movement: Movement) {
		if !self.movement.contains(&movement) {
			self.movement.push(movement);
		}
	}

	pub fn stop_event(&mut self, movement: Movement) {
		if self.movement.contains(&movement) {
			self.movement.remove(self.movement.iter().position(|x| x == &movement).unwrap());
		}
	}

	#[allow(dead_code)]
	pub fn is_moving(&self) -> bool {
		!self.movement.is_empty()
	}

	pub fn get_movement_vector(&self) -> Vector<f32> {
		let mut movement_vector = Vector::from(&[0.0, 0.0, 0.0]);
		for movement in &self.movement {
			match movement {
				Movement::Forward => movement_vector[2] += 1.0,
				Movement::Backward => movement_vector[2] -= 1.0,
				Movement::Left => movement_vector[0] -= 1.0,
				Movement::Right => movement_vector[0] += 1.0,
				Movement::Up => movement_vector[1] += 1.0,
				Movement::Down => movement_vector[1] -= 1.0,
				_ => (),
			}
		}
		movement_vector
	}

	pub fn get_rotation_vector(&self) -> Vector<f32> {
		let mut rotation_vector = Vector::from(&[0.0, 0.0, 0.0]);
		for movement in &self.movement {
			match movement {
				Movement::RotateLeft => rotation_vector[0] -= 1.0,
				Movement::RotateRight => rotation_vector[0] += 1.0,
				Movement::RotateUp => rotation_vector[1] += 1.0,
				Movement::RotateDown => rotation_vector[1] -= 1.0,
				Movement::RotateWest => rotation_vector[2] += 1.0,
				Movement::RotateEast => rotation_vector[2] -= 1.0,
				_ => (),
			}
		}
		rotation_vector
	}

	pub fn mouse_moved(&self, position: PhysicalPosition<f64>, window_extent: &(f32, f32), camera: &mut Camera) {
		let x = position.x as f32 - window_extent.0 / 2.0;
		let y = position.y as f32 - window_extent.1 / 2.0;

		camera.rotate_from_vector3(Vector::from(&[x, -y, 0.0]), 0.1);
	}
}