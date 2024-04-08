use core::time::Duration;

use crate::peripherals::system_time::SystemTime;

pub struct MockSystemTime
{
	pub current_time: Duration,
}

impl SystemTime for MockSystemTime
{
	fn now(&self) -> Duration
	{
		self.current_time
	}

	fn delay(&self, _: Duration)
	{
		todo!()
	}
}
