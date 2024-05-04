pub struct MyF32(pub f32);

impl Into<f32> for MyF32
{
	fn into(self) -> f32 {
		return self.0;
	}
}

impl From<i8> for MyF32
{
	fn from(value: i8) -> Self {
		return MyF32(value as f32);
	}
}

impl From<i16> for MyF32
{
	fn from(value: i16) -> Self {
		return MyF32(value as f32);
	}
}

impl From<i32> for MyF32
{
	fn from(value: i32) -> Self {
		return MyF32(value as f32);
	}
}

impl From<i64> for MyF32
{
	fn from(value: i64) -> Self {
		return MyF32(value as f32);
	}
}

impl From<i128> for MyF32
{
	fn from(value: i128) -> Self {
		return MyF32(value as f32);
	}
}

impl From<isize> for MyF32
{
	fn from(value: isize) -> Self {
		return MyF32(value as f32);
	}
}

impl From<u8> for MyF32
{
	fn from(value: u8) -> Self {
		return MyF32(value as f32);
	}
}

impl From<u16> for MyF32
{
	fn from(value: u16) -> Self {
		return MyF32(value as f32);
	}
}

impl From<u32> for MyF32
{
	fn from(value: u32) -> Self {
		return MyF32(value as f32);
	}
}

impl From<u64> for MyF32
{
	fn from(value: u64) -> Self {
		return MyF32(value as f32);
	}
}

impl From<u128> for MyF32
{
	fn from(value: u128) -> Self {
		return MyF32(value as f32);
	}
}

impl From<usize> for MyF32
{
	fn from(value: usize) -> Self {
		return MyF32(value as f32);
	}
}

impl From<f32> for MyF32
{
	fn from(value: f32) -> Self {
		return MyF32(value);
	}
}

impl From<f64> for MyF32
{
	fn from(value: f64) -> Self {
		return MyF32(value as f32);
	}
}

pub trait IntoF32 {
	fn into_f32(self) -> f32;
}

impl IntoF32 for i8 {
	fn into_f32(self) -> f32 {
		let v: MyF32 = self.into();
		return v.0;
	}
}

impl IntoF32 for i16 {
	fn into_f32(self) -> f32 {
		let v: MyF32 = self.into();
		return v.0;
	}
}

impl IntoF32 for i32 {
	fn into_f32(self) -> f32 {
		let v: MyF32 = self.into();
		return v.0;
	}
}

impl IntoF32 for i64 {
	fn into_f32(self) -> f32 {
		let v: MyF32 = self.into();
		return v.0;
	}
}

impl IntoF32 for i128 {
	fn into_f32(self) -> f32 {
		let v: MyF32 = self.into();
		return v.0;
	}
}

impl IntoF32 for isize {
	fn into_f32(self) -> f32 {
		let v: MyF32 = self.into();
		return v.0;
	}
}

impl IntoF32 for u8 {
	fn into_f32(self) -> f32 {
		let v: MyF32 = self.into();
		return v.0;
	}
}

impl IntoF32 for u16 {
	fn into_f32(self) -> f32 {
		let v: MyF32 = self.into();
		return v.0;
	}
}

impl IntoF32 for u32 {
	fn into_f32(self) -> f32 {
		let v: MyF32 = self.into();
		return v.0;
	}
}

impl IntoF32 for u64 {
	fn into_f32(self) -> f32 {
		let v: MyF32 = self.into();
		return v.0;
	}
}

impl IntoF32 for u128 {
	fn into_f32(self) -> f32 {
		let v: MyF32 = self.into();
		return v.0;
	}
}

impl IntoF32 for usize {
	fn into_f32(self) -> f32 {
		let v: MyF32 = self.into();
		return v.0;
	}
}

impl IntoF32 for f32 {
	fn into_f32(self) -> f32 {
		let v: MyF32 = self.into();
		return v.0;
	}
}

impl IntoF32 for f64 {
	fn into_f32(self) -> f32 {
		let v: MyF32 = self.into();
		return v.0;
	}
}
