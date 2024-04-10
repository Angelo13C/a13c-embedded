use embedded_hal::digital::{ErrorType, InputPin};

use super::{button::Button, potentiometer::Potentiometer};
use crate::{
	peripherals::adc::{Adc, AdcPin, ReadPercentageError},
	utils::math::Percentage,
};

pub struct Joystick<A: Adc, PX: AdcPin<A>, PY: AdcPin<A>, PZ: InputPin>
{
	x_axis: Potentiometer<A, PX>,
	y_axis: Potentiometer<A, PY>,
	button: Button<PZ>,
}

impl<A: Adc, PX: AdcPin<A>, PY: AdcPin<A>, PZ: InputPin> Joystick<A, PX, PY, PZ>
{
	pub fn new(x_pin: PX, y_pin: PY, z_pin: PZ) -> Self
	{
		Self {
			x_axis: Potentiometer::new(x_pin),
			y_axis: Potentiometer::new(y_pin),
			button: Button::new(z_pin),
		}
	}

	pub fn x(&mut self, adc: &mut A) -> Result<Percentage, ReadPercentageError<A, PX>>
	{
		self.x_axis.get_value(adc)
	}
	pub fn y(&mut self, adc: &mut A) -> Result<Percentage, ReadPercentageError<A, PY>>
	{
		self.y_axis.get_value(adc)
	}

	pub fn z(&mut self) -> Result<bool, <PZ as ErrorType>::Error>
	{
		self.button.is_pressed()
	}

	pub fn coords(&mut self, adc: &mut A) -> Result<JoystickCoords, ()>
	{
		Ok(JoystickCoords {
			x: self.x(adc).map_err(|_| ())?,
			y: self.y(adc).map_err(|_| ())?,
			z: self.z().map_err(|_| ())?,
		})
	}
}

pub struct JoystickCoords
{
	pub x: Percentage,
	pub y: Percentage,
	pub z: bool,
}
