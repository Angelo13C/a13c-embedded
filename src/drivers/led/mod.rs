mod pwm;
mod rgb;

use embedded_hal::digital::{ErrorType, OutputPin, StatefulOutputPin};
pub use pwm::*;
pub use rgb::*;

/// A [`light-emitting diode`] connected to the microcontroller through the `P` pin, which allows to control it.
///
/// [`light-emitting diode`]: <https://en.wikipedia.org/wiki/Light-emitting_diode>
pub struct Led<P: OutputPin>
{
	pin: P,
}

impl<P: OutputPin> Led<P>
{
	/// Returns a [`Led`] that can control its on/off state through the provided `pin`.
	pub fn new(pin: P) -> Self
	{
		Self { pin }
	}

	/// Turns on the led (setting the pin high).
	///
	/// Returns an error if there was an error controlling the output state of the pin.
	pub fn turn_on(&mut self) -> Result<(), <P as ErrorType>::Error>
	{
		self.pin.set_high()
	}
	/// Turns off the led (setting the pin low).
	///
	/// Returns an error if there was an error controlling the output state of the pin.
	pub fn turn_off(&mut self) -> Result<(), <P as ErrorType>::Error>
	{
		self.pin.set_low()
	}

	/// If `on` is `true`, it calls [`Self::turn_on`]. Otherwise it calls [`Self::turn_off`].
	///
	/// Returns an error if there was an error controlling the output state of the pin.
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
	/// Toggles the on/off state of the led.
	///
	/// Returns an error if there was an error controlling the output state of the pin.
	pub fn toggle(&mut self) -> Result<(), <P as ErrorType>::Error>
	{
		self.pin.toggle()
	}

	/// Returns `true` if the led is on (the pin is set to high), otherwise returns `false`.
	pub fn is_on(&mut self) -> Result<bool, <P as ErrorType>::Error>
	{
		self.pin.is_set_high()
	}
}
