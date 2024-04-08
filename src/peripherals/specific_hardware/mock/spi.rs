use embedded_hal::spi::{ErrorType, Operation, SpiDevice};

use super::MockSpiError;

extern crate alloc;
use alloc::vec::*;

pub enum MockSpi<Word: Copy + 'static = u8>
{
	Ok
	{
		read_operations: Vec<Vec<Word>>,
		write_operations: Vec<Vec<Word>>,
		delays_operations: Vec<u32>,
	},
	Err(MockSpiError),
}
impl<Word: Copy + 'static> SpiDevice<Word> for MockSpi<Word>
{
	fn transaction(&mut self, new_operations: &mut [Operation<'_, Word>]) -> Result<(), Self::Error>
	{
		match self
		{
			MockSpi::Ok {
				read_operations,
				write_operations,
				delays_operations,
			} =>
			{
				new_operations.iter_mut().for_each(|operation| match operation
				{
					Operation::Read(operation) => match read_operations.pop()
					{
						Some(read_operation) =>
						{
							operation
								.iter_mut()
								.enumerate()
								.for_each(|(i, byte)| match read_operation.get(i)
								{
									Some(new_byte) => *byte = *new_byte,
									None => todo!(),
								})
						},
						None => todo!(),
					},
					Operation::Write(operation) => write_operations.push(operation.to_vec()),
					Operation::Transfer(_, _) => todo!(),
					Operation::TransferInPlace(_) => todo!(),
					Operation::DelayNs(operation) => delays_operations.push(*operation),
				});
				Ok(())
			},
			MockSpi::Err(error) => Err(*error),
		}
	}
}

impl<Word: Copy + 'static> ErrorType for MockSpi<Word>
{
	type Error = MockSpiError;
}
