use core::time::Duration;

use crate::peripherals::time::system_time::SystemTime;

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
}
