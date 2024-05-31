use core::fmt::Debug;

pub use time;
use time::{Date, Time};

pub trait RealTimeClock
{
	type Error: Debug;

	fn now(&self) -> Result<(Date, Time), Self::Error>;
}
