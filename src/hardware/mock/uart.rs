use crate::{peripherals::uart::Uart, utils::physical_quantities::duration::SmallDuration};

extern crate alloc;
use alloc::vec::*;

pub struct MockUart
{
	read_buffer: Vec<u8>,
	write_buffer: Vec<u8>,
}
impl Uart for MockUart
{
	type Error = ();

	fn read(&mut self, buf: &mut [u8], timeout: SmallDuration) -> Result<usize, Self::Error>
	{
		let read_bytes_count = self.read_buffer.len();
		buf.iter_mut()
			.take(read_bytes_count)
			.enumerate()
			.for_each(|(i, byte)| *byte = self.read_buffer[i]);

		self.read_buffer = self.read_buffer.split_off(read_bytes_count);

		Ok(read_bytes_count)
	}

	fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error>
	{
		self.write_buffer.extend_from_slice(buf);
		Ok(buf.len())
	}

	fn flush_read(&mut self) -> Result<(), Self::Error>
	{
		self.read_buffer.clear();
		Ok(())
	}
}
