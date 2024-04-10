use crate::{peripherals::pwm::PwmPin, utils::math::Percentage};

pub struct LedPwm<P: PwmPin>
{
	pin: P,
}

impl<P: PwmPin> LedPwm<P>
{
	pub fn new(pin: P) -> Self
	{
		Self { pin }
	}

	pub fn turn_on(&mut self) -> Result<(), <P as PwmPin>::Error>
	{
		self.pin.set_duty_cycle(Percentage::FULL)
	}
	pub fn turn_off(&mut self) -> Result<(), <P as PwmPin>::Error>
	{
		self.pin.set_duty_cycle(Percentage::ZERO)
	}

	pub fn set_intensity(&mut self, intensity: Percentage) -> Result<(), <P as PwmPin>::Error>
	{
		self.pin.set_duty_cycle(intensity)
	}

	pub fn get_intensity(&self) -> Percentage
	{
		self.pin.get_duty_cycle()
	}
}
