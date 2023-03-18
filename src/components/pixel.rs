#[derive(Clone)]
pub struct Pixel{
	pub x: f64,
	pub y: f64,
	pub rgba: [u8;4]
}

impl Pixel {
	pub fn new(x: f64, y: f64, red: u8, green: u8, blue: u8, alpha: u8) -> Self {
		Self {
			x,
			y,
			rgba: [red, green, blue, alpha]
		}
	}
}