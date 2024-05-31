use embedded_hal::digital::{ErrorType, InputPin};

use super::MockDigitalError;

pub enum MockInputPin
{
	Ok
	{
		is_high: bool,
	},
	Err(MockDigitalError),
}

impl InputPin for MockInputPin
{
	fn is_high(&mut self) -> Result<bool, Self::Error>
	{
		match self
		{
			MockInputPin::Ok { is_high } => Ok(*is_high),
			MockInputPin::Err(error) => Err(*error),
		}
	}

	fn is_low(&mut self) -> Result<bool, Self::Error>
	{
		match self
		{
			MockInputPin::Ok { is_high } => Ok(!*is_high),
			MockInputPin::Err(error) => Err(*error),
		}
	}
}

impl ErrorType for MockInputPin
{
	type Error = MockDigitalError;
}
