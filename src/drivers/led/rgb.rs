use embedded_graphics::pixelcolor::{Rgb888, RgbColor};

use super::LedPwm;
use crate::{peripherals::pwm::PwmPin, utils::math::Percentage};

/// An RGB [`light-emitting diode`] connected to the microcontroller through the `PRed`, `PGreen`, `PBlue` PWM pins, which allows
/// to control it.
///
/// [`light-emitting diode`]: <https://en.wikipedia.org/wiki/Light-emitting_diode>
pub struct LedRgb<PRed: PwmPin, PGreen: PwmPin, PBlue: PwmPin>
{
	red: LedPwm<PRed>,
	green: LedPwm<PGreen>,
	blue: LedPwm<PBlue>,
}
impl<PRed: PwmPin, PGreen: PwmPin, PBlue: PwmPin> LedRgb<PRed, PGreen, PBlue>
{
	/// Returns a [`Led`] that can control its color state through the provided pins.
	pub fn new(red_pin: PRed, green_pin: PGreen, blue_pin: PBlue) -> Self
	{
		Self {
			red: LedPwm::new(red_pin),
			green: LedPwm::new(green_pin),
			blue: LedPwm::new(blue_pin),
		}
	}

	/// Turns off the led (setting all the 3 channels to have a `0%` duty cycle).
	pub fn turn_off(&mut self) -> Result<(), SetColorError<PRed, PGreen, PBlue>>
	{
		self.set_color(Rgb888::new(0, 0, 0))
	}

	/// Sets the color of the led.
	pub fn set_color<Color: RgbColor>(&mut self, color: Color) -> Result<(), SetColorError<PRed, PGreen, PBlue>>
	{
		self.red
			.set_intensity(Percentage::from_0_to_1(color.r() as f32 / Color::MAX_R as f32).unwrap())
			.map_err(SetColorError::Red)?;
		self.green
			.set_intensity(Percentage::from_0_to_1(color.g() as f32 / Color::MAX_G as f32).unwrap())
			.map_err(SetColorError::Green)?;
		self.blue
			.set_intensity(Percentage::from_0_to_1(color.b() as f32 / Color::MAX_B as f32).unwrap())
			.map_err(SetColorError::Blue)?;

		Ok(())
	}

	/// Sets the previously set color of the led.
	pub fn get_color(&self) -> Rgb888
	{
		Rgb888::new(
			(self.red.get_intensity().into_0_to_1() * u8::MAX as f32) as u8,
			(self.green.get_intensity().into_0_to_1() * u8::MAX as f32) as u8,
			(self.blue.get_intensity().into_0_to_1() * u8::MAX as f32) as u8,
		)
	}
}

/// An error returned from [`LedRgb::set_color`] or [`LedRgb::turn_off`].
pub enum SetColorError<PRed: PwmPin, PGreen: PwmPin, PBlue: PwmPin>
{
	Red(PRed::Error),
	Green(PGreen::Error),
	Blue(PBlue::Error),
}
