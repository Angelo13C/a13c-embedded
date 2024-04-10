use embedded_graphics::pixelcolor::{Rgb888, RgbColor};

use super::LedPwm;
use crate::{peripherals::pwm::PwmPin, utils::math::Percentage};

pub struct LedRgb<PRed: PwmPin, PGreen: PwmPin, PBlue: PwmPin>
{
	red: LedPwm<PRed>,
	green: LedPwm<PGreen>,
	blue: LedPwm<PBlue>,
}
impl<PRed: PwmPin, PGreen: PwmPin, PBlue: PwmPin> LedRgb<PRed, PGreen, PBlue>
{
	pub fn new(red_pin: PRed, green_pin: PGreen, blue_pin: PBlue) -> Self
	{
		Self {
			red: LedPwm::new(red_pin),
			green: LedPwm::new(green_pin),
			blue: LedPwm::new(blue_pin),
		}
	}

	pub fn turn_off(&mut self)
	{
		self.set_color(Rgb888::new(0, 0, 0))
	}

	pub fn set_color<Color: RgbColor>(&mut self, color: Color)
	{
		self.red
			.set_intensity(Percentage::from_0_to_1(color.r() as f32 / Color::MAX_R as f32).unwrap());
		self.green
			.set_intensity(Percentage::from_0_to_1(color.g() as f32 / Color::MAX_G as f32).unwrap());
		self.blue
			.set_intensity(Percentage::from_0_to_1(color.b() as f32 / Color::MAX_B as f32).unwrap());
	}

	pub fn get_color(&self) -> Rgb888
	{
		Rgb888::new(
			(self.red.get_intensity().into_0_to_1() * u8::MAX as f32) as u8,
			(self.green.get_intensity().into_0_to_1() * u8::MAX as f32) as u8,
			(self.blue.get_intensity().into_0_to_1() * u8::MAX as f32) as u8,
		)
	}
}
