use core::marker::PhantomData;

use crate::{peripherals::adc::*, utils::math::Percentage};

pub struct Potentiometer<A: Adc, P: AdcPin<A>>
{
	pin: P,
	_a: PhantomData<A>,
}

impl<A: Adc, P: AdcPin<A>> Potentiometer<A, P>
{
	pub fn new(pin: P) -> Self
	{
		Self { pin, _a: PhantomData }
	}

	pub fn get_value(&mut self, adc: &mut A) -> Result<Percentage, ReadPercentageError<A, P>>
	{
		self.pin.read_percentage(adc)
	}
}
