use num_traits::ops::mul_add::MulAdd;
// Module: fma
// fma => a * b + c
// sfma => a * b + self to self
pub trait Fma {
	fn fma(a: Self, b: Self, c: Self) -> Self;
	fn sfma(&mut self, a: Self, b: Self);
}

impl<T: MulAdd<Output = T> + Copy> Fma for T {
	fn fma(a: T, b: T, c: T) -> T {
		a.mul_add(b, c)
	}
	fn sfma(&mut self, a: T, b: T) {
		*self = a.mul_add(b, *self);
	}
}
