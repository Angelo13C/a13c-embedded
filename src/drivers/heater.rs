use crate::{peripherals::pwm::PwmPin, utils::math::Percentage};

/// An heater connected to the microcontroller that can be controlled using a PWM signal on the `P` pin.
pub struct PwmHeater<P: PwmPin>
{
	pin: P,
}

impl<P: PwmPin> PwmHeater<P>
{
	/// Returns a [`PwmHeater`] that can control its heat percentage through the provided `pin`.
	pub fn new(pin: P) -> Self
	{
		Self { pin }
	}

	/// Sets the `percentage` of current to give to the heater.
	pub fn set_heat_percentage(&mut self, percentage: Percentage) -> Result<(), <P as PwmPin>::Error>
	{
		self.pin.set_duty_cycle(percentage)
	}
}
