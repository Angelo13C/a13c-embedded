use core::fmt::Debug;

use embedded_hal::spi::SpiDevice;

use crate::utils::physical_quantities::temperature::Temperature;

pub struct MAX6675<Spi: SpiDevice<u8>>
{
	spi: Spi,
}

impl<Spi: SpiDevice<u8>> MAX6675<Spi>
{
	pub const SENSITIVITY: Temperature = Temperature::from_kelvin(0.25);

	pub fn new(spi: Spi) -> Self
	{
		Self { spi }
	}

	pub fn read_temperature(&mut self) -> Result<Temperature, ReadError<Spi>>
	{
		let mut read_bytes = [0_u8; 2];
		self.spi.transfer(&mut read_bytes, &[0_u8; 2]).map_err(ReadError::Spi)?;

		let read_value = u16::from_be_bytes(read_bytes);

		if read_value & (1 << 2) != 0
		{
			return Err(ReadError::OpenThermocouple);
		}

		let temperature = Temperature::from_celsius(Self::SENSITIVITY.as_kelvin() * (read_value >> 3) as f32);
		Ok(temperature)
	}
}

pub enum ReadError<Spi: SpiDevice<u8>>
{
	Spi(Spi::Error),
	OpenThermocouple,
}

impl<Spi: SpiDevice<u8>> Debug for ReadError<Spi>
{
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
	{
		match self
		{
			Self::Spi(arg0) => f.debug_tuple("Spi").field(arg0).finish(),
			Self::OpenThermocouple => write!(f, "OpenCircuit"),
		}
	}
}
