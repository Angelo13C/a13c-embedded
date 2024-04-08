use embedded_hal::digital::{ErrorType, OutputPin};

use super::MockDigitalError;

pub enum MockOutputPin
{
	Ok
	{
		is_high: bool,
	},
	Err(MockDigitalError),
}

impl MockOutputPin
{
	pub fn set(&mut self, value: bool) -> Result<(), <Self as ErrorType>::Error>
	{
		match self
		{
			MockOutputPin::Ok { is_high } =>
			{
				*is_high = value;
				Ok(())
			},
			MockOutputPin::Err(error) => Err(*error),
		}
	}
}

impl OutputPin for MockOutputPin
{
	fn set_high(&mut self) -> Result<(), Self::Error>
	{
		self.set(true)
	}

	fn set_low(&mut self) -> Result<(), Self::Error>
	{
		self.set(false)
	}
}

impl ErrorType for MockOutputPin
{
	type Error = MockDigitalError;
}
