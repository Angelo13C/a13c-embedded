use core::fmt::Debug;

use esp_idf_hal::uart::UartDriver;
use esp_idf_sys::{EspError, TickType_t};

use crate::{peripherals::uart::Uart, utils::physical_quantities::duration::SmallDuration};

pub struct UARTDriver<'d>(pub UartDriver<'d>);

const fn duration_to_rtos_ticks(duration: SmallDuration) -> TickType_t
{
	duration.as_millis() as TickType_t
}

impl<'d> Uart for UARTDriver<'d>
{
	type Error = EspError;

	fn read(&mut self, buf: &mut [u8], timeout: SmallDuration) -> Result<usize, Self::Error>
	{
		self.0.read(buf, duration_to_rtos_ticks(timeout))
	}

	fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error>
	{
		self.0.write(buf)
	}

	fn flush_read(&mut self) -> Result<(), Self::Error>
	{
		self.0.clear_rx()
	}
}

impl<'d> Debug for UARTDriver<'d>
{
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
	{
		f.debug_tuple("UARTDriver").finish()
	}
}
