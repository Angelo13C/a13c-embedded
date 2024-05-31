use super::MockEmptyError;
use crate::{
	peripherals::pwm::PwmPin,
	utils::{math::Percentage, physical_quantities::frequency::Frequency},
};

pub enum MockPwmPin
{
	Ok
	{
		duty_cycle: Percentage,
		frequency: Frequency,
	},
	Err,
}
impl PwmPin for MockPwmPin
{
	type Error = MockEmptyError;

	fn get_duty_cycle(&self) -> Percentage
	{
		match self
		{
			MockPwmPin::Ok {
				duty_cycle,
				frequency: _,
			} => *duty_cycle,
			MockPwmPin::Err => panic!("Invalid"),
		}
	}

	fn set_duty_cycle(&mut self, new_duty_cycle: Percentage) -> Result<(), Self::Error>
	{
		match self
		{
			MockPwmPin::Ok {
				duty_cycle,
				frequency: _,
			} =>
			{
				*duty_cycle = new_duty_cycle;
				Ok(())
			},
			MockPwmPin::Err => Err(MockEmptyError),
		}
	}

	fn set_frequency(&mut self, new_frequency: Frequency) -> Result<(), Self::Error>
	{
		match self
		{
			MockPwmPin::Ok {
				duty_cycle: _,
				frequency,
			} =>
			{
				*frequency = new_frequency;
				Ok(())
			},
			MockPwmPin::Err => Err(MockEmptyError),
		}
	}
}
