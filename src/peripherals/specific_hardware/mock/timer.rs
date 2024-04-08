use crate::{
	peripherals::timer::{Timer, TimerAdditionalFunctionality},
	utils::physical_quantities::frequency::Frequency,
};

extern crate alloc;
use alloc::boxed::Box;

pub struct MockTimer
{
	pub clock_frequency: Frequency,
	pub additional_functionality: MockTimerAdditionalFunctionality,
	pub is_alarm_enabled: bool,
	pub callback: Box<dyn FnMut()>,
}
impl Timer for MockTimer
{
	type Error = ();
	type AdditionalFunctionality = MockTimerAdditionalFunctionality;

	fn get_additional_functionality(&self) -> Self::AdditionalFunctionality
	{
		self.additional_functionality.clone()
	}

	fn get_clock_frequency(&self) -> Frequency
	{
		self.clock_frequency
	}

	unsafe fn on_alarm(&mut self, callback: impl FnMut() + 'static) -> Result<(), Self::Error>
	{
		self.callback = Box::new(callback);
		Ok(())
	}

	fn enable_alarm(&mut self, value: bool) -> Result<(), Self::Error>
	{
		self.is_alarm_enabled = value;
		Ok(())
	}
}

#[derive(Debug, Clone)]
pub struct MockTimerAdditionalFunctionality;
impl TimerAdditionalFunctionality for MockTimerAdditionalFunctionality
{
	type Error = ();

	fn set_alarm(&mut self, _: core::time::Duration) -> Result<(), Self::Error>
	{
		todo!()
	}

	fn get_time(&self) -> Result<core::time::Duration, Self::Error>
	{
		todo!()
	}

	fn set_alarm_in_ticks(&mut self, ticks: u64) -> Result<(), Self::Error>
	{
		todo!()
	}

	fn get_time_in_ticks(&self) -> Result<u64, Self::Error>
	{
		todo!()
	}
}
