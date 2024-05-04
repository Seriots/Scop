
use std::f32::consts::PI;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use num_traits::pow;

use crate::utils::NumberUtils;
use crate::utils::Fma;
use crate::Lerp;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};

#[derive(Debug, Clone, Copy)]
pub struct Complex {
	real: f32,
	imag: f32,
}

impl Complex {
	pub fn new(real: f32, imag: f32) -> Self {
		Self { real, imag }
	}

	pub fn conjugate(&mut self) {
		self.imag *= -1.0;
	}
}

impl Display for Complex {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{} + {}i", self.real, self.imag)
	}
}

impl NumberUtils for Complex {
	fn one() -> Self {
		Self::new(1.0, 0.0)
	}

	fn approx_zero(&self) -> bool {
		self.real.approx_zero() && self.imag.approx_zero()
	}

	fn absolute(&self) -> Self {
		Self::new(self.real.absolute(), self.imag.absolute())
	}

	fn power(&self, n: usize) -> Self {
		let r = (self.real * self.real + self.imag * self.imag).powf(n as f32 / 2.);
		let theta = (self.imag / self.real).atan() * n as f32;
		Self::new(r * theta.cos(), r * theta.sin())
	}

	fn squarert(&self) -> Self {
		let r = (self.real * self.real + self.imag * self.imag).powf(0.5).powf(0.5);
		let theta = (self.imag / self.real).atan() / 2.;
		if self.real < 0.0 {
			Self::new(r * theta.sin(), r * theta.cos())
		} else {
			Self::new(r * theta.cos(), r * theta.sin())
		}
	}
}

impl Default for Complex {
	fn default() -> Self {
		Self::new(0.0, 0.0)
	}
}

impl Add for Complex {
	type Output = Self;

	fn add(self, rhs: Self) -> Self {
		Self::new(self.real + rhs.real, self.imag + rhs.imag)
	}
}

impl AddAssign for Complex {
	fn add_assign(&mut self, rhs: Self) {
		self.real += rhs.real;
		self.imag += rhs.imag;
	}
}

impl Sub for Complex {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self {
		Self::new(self.real - rhs.real, self.imag - rhs.imag)
	}
}

impl SubAssign for Complex {
	fn sub_assign(&mut self, rhs: Self) {
		self.real -= rhs.real;
		self.imag -= rhs.imag;
	}
}

impl Mul for Complex {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self {
		Self::new(
			self.real * rhs.real - self.imag * rhs.imag,
			self.real * rhs.imag + self.imag * rhs.real,
		)
	}
}

impl MulAssign for Complex {
	fn mul_assign(&mut self, rhs: Self) {
		*self = Self::new(
			self.real * rhs.real - self.imag * rhs.imag,
			self.real * rhs.imag + self.imag * rhs.real,
		);
	}
}

impl Div for Complex {
	type Output = Self;

	fn div(self, rhs: Self) -> Self {
		let denom = rhs.real * rhs.real + rhs.imag * rhs.imag;
		Self::new(
			(self.real * rhs.real + self.imag * rhs.imag) / denom,
			(self.imag * rhs.real - self.real * rhs.imag) / denom,
		)
	}
}

impl DivAssign for Complex {
	fn div_assign(&mut self, rhs: Self) {
		let denom = rhs.real * rhs.real + rhs.imag * rhs.imag;
		*self = Self::new(
			(self.real * rhs.real + self.imag * rhs.imag) / denom,
			(self.imag * rhs.real - self.real * rhs.imag) / denom,
		);
	}
}

impl Neg for Complex {
	type Output = Self;

	fn neg(self) -> Self {
		Self::new(-self.real, -self.imag)
	}
}

impl Fma for Complex {
	fn fma(a: Self, b: Self, c: Self) -> Self {
		a * b + c
	}

	fn sfma(&mut self, a: Self, b: Self) {
		*self += a * b;
	}
}

impl PartialEq for Complex {
	fn eq(&self, other: &Self) -> bool {
		(self.real - other.real).abs() < 1e-6 && (self.imag - other.imag).abs() < 1e-6
	}
}

impl Eq for Complex {}

impl PartialOrd for Complex {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		if self.real < other.real {
			return Some(std::cmp::Ordering::Less);
		} else if self.real > other.real {
			return Some(std::cmp::Ordering::Greater);
		} else {
			if self.imag < other.imag {
				return Some(std::cmp::Ordering::Less);
			} else if self.imag > other.imag {
				return Some(std::cmp::Ordering::Greater);
			} else {
				return Some(std::cmp::Ordering::Equal);
			}
		}
	}
}

impl Ord for Complex {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		if self.real < other.real {
			return std::cmp::Ordering::Less;
		} else if self.real > other.real {
			return std::cmp::Ordering::Greater;
		} else {
			if self.imag < other.imag {
				return std::cmp::Ordering::Less;
			} else if self.imag > other.imag {
				return std::cmp::Ordering::Greater;
			} else {
				return std::cmp::Ordering::Equal;
			}
		}
	}
}

impl Into<f32> for Complex {
	fn into(self) -> f32 {
		self.real
	}
}

impl From<f32> for Complex {
	fn from(f: f32) -> Self {
		Self::new(f, 0.0)
	}
}

impl Lerp for Complex {
	fn lerp(self, v: Self, t: f32) -> Self {
		Fma::fma(v - self, t.into(), self)
	}
}

//Debug + Display + NumberUtils + Clone + Default + Fma + IntoF32 + AddAssign + Add<Output = K> + SubAssign + Sub<Output = K> + MulAssign + DivAssign + Mul<Output = K> + Div<Output = K> + Neg<Output = K> + PartialEq