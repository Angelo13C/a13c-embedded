use core::convert::Infallible;

use embedded_hal::digital::OutputPin;

use super::PwmPin;
use crate::utils::{
	math::Percentage,
	physical_quantities::{duration::SmallDuration, frequency::Frequency},
};

/// # Warning
/// You **MUST** continuously call [`Self::tick`] to make this work.
pub struct SystemTimePwm<P: OutputPin>
{
	pin: P,
	duty_cycle: Percentage,
	single_cycle_duration: SmallDuration,
	current_time: SmallDuration,
}

impl<P: OutputPin> SystemTimePwm<P>
{
	pub fn new(mut pin: P, single_cycle_duration: SmallDuration) -> Result<Self, P::Error>
	{
		pin.set_low()?;

		Ok(Self {
			pin,
			duty_cycle: Percentage::ZERO,
			single_cycle_duration,
			current_time: SmallDuration::ZERO,
		})
	}

	pub fn tick(&mut self, delta_time: SmallDuration) -> Result<(), P::Error>
	{
		self.current_time += delta_time;
		while self.current_time >= self.single_cycle_duration
		{
			self.current_time -= self.single_cycle_duration;
		}

		match self.current_time > self.single_cycle_duration * self.duty_cycle.into_0_to_1()
		{
			true => self.pin.set_high(),
			false => self.pin.set_low(),
		}
	}
}

impl<P: OutputPin> PwmPin for SystemTimePwm<P>
{
	type Error = Infallible;

	fn get_duty_cycle(&self) -> Percentage
	{
		self.duty_cycle
	}

	fn set_duty_cycle(&mut self, percentage: Percentage) -> Result<(), Self::Error>
	{
		self.duty_cycle = percentage;
		Ok(())
	}

	fn set_frequency(&mut self, frequency: Frequency) -> Result<(), Self::Error>
	{
		self.single_cycle_duration = frequency.into();
		Ok(())
	}
}
