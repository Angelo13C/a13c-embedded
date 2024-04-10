mod pwm;
mod rgb;

use embedded_hal::digital::{ErrorType, OutputPin, StatefulOutputPin};
pub use pwm::*;
pub use rgb::*;

pub struct Led<P: OutputPin>
{
	pin: P,
}

impl<P: OutputPin> Led<P>
{
	pub fn new(pin: P) -> Self
	{
		Self { pin }
	}

	pub fn turn_on(&mut self) -> Result<(), <P as ErrorType>::Error>
	{
		self.pin.set_high()
	}
	pub fn turn_off(&mut self) -> Result<(), <P as ErrorType>::Error>
	{
		self.pin.set_low()
	}
	pub fn set_to(&mut self, on: bool) -> Result<(), <P as ErrorType>::Error>
	{
		if on
		{
			self.turn_on()
		}
		else
		{
			self.turn_off()
		}
	}
}

impl<P: StatefulOutputPin> Led<P>
{
	pub fn toggle(&mut self) -> Result<(), <P as ErrorType>::Error>
	{
		self.pin.toggle()
	}

	pub fn is_on(&mut self) -> Result<bool, <P as ErrorType>::Error>
	{
		self.pin.is_set_high()
	}
}
