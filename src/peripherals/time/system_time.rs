//! Module for handling system-time related operations.
//!
//! ## Usage
//! The [`Clock`] struct keeps track of the system time and the delta time between ticks, while the [`SystemTime`]
//! trait is used as an abstraction layer for the system time source of a specific microcontroller.
//!
//! ## Example
//!
//! ```rust
//! use core::time::Duration;
//! use a13c_embedded::peripherals::time::system_time::*;
//!
//! let system_time = MockSystemTime(Duration::from_secs(0));
//! let mut clock = Clock::new(system_time);
//! assert_eq!(clock.get_delta_time(), Duration::ZERO);
//!
//! // This is how you could implement an interrupt pin for your specific hardware
//! struct MockSystemTime(Duration);
//!
//! impl SystemTime for MockSystemTime {
//!     fn now(&self) -> Duration {
//!         self.0
//!     }
//! }
//! ```

use core::{fmt::Debug, time::Duration};

/// A clock used to keep track of the [`SystemTime`] and the delta time between ticks of the printer.
pub struct Clock<T: SystemTime>
{
	system_time: T,
	last_tick_time: Duration,
}

impl<T: SystemTime> Clock<T>
{
	/// Returns a [`Clock`] that has just been [`ticked`].
	///
	/// # Examples
	/// ```
	/// # /*
	/// let system_time = //...
	/// let mut clock = Clock::new(system_time);
	/// assert_eq!(clock.get_delta_time(), Duration::ZERO);
	/// # */
	/// ```
	///
	/// [`ticked`]: `Self::tick`
	pub fn new(system_time: T) -> Self
	{
		let mut self_ = Self {
			system_time,
			last_tick_time: Duration::ZERO,
		};
		self_.tick();

		self_
	}

	/// Saves the [`current time`] in the clock so that you can [`get the delta time`] later.
	///
	/// [`current time`]: `Self::get_elapsed_time`
	/// [`get the delta time`]: `Self::get_delta_time`
	pub fn tick(&mut self)
	{
		self.last_tick_time = self.system_time.now();
	}

	/// Returns the amount of time this microcontroller has been running for.
	pub fn get_elapsed_time(&self) -> Duration
	{
		self.system_time.now()
	}

	/// Returns the delta time between the [`current time`] and the last time you called [`tick`].
	///
	/// # Examples
	/// ```
	/// # /*
	/// let system_time = //...
	/// let mut clock = Clock::new(system_time);
	///
	/// // You do some stuff here that takes 1ms...
	///
	/// assert_eq!(clock.get_delta_time(), Duration::from_millis(1));
	///
	/// // You do some stuff here that takes 3ms...
	///
	/// assert_eq!(clock.get_delta_time(), Duration::from_millis(1 + 3));
	///
	/// clock.tick();
	/// assert_eq!(clock.get_delta_time(), Duration::ZERO);
	/// # */
	/// ```
	///
	/// [`current time`]: `Self::get_elapsed_time`
	/// [`tick`]: `Self::tick`
	pub fn get_delta_time(&self) -> Duration
	{
		self.get_elapsed_time() - self.last_tick_time
	}
}

/// A clock in the microcontroller that can keep time.
pub trait SystemTime
{
	/// Returns the amount of time this microcontroller has been running for.
	fn now(&self) -> Duration;
}

impl<T: SystemTime> Debug for Clock<T>
{
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
	{
		f.debug_struct("Clock")
			.field("elapsed_time", &self.get_elapsed_time())
			.field("last_tick_time", &self.last_tick_time)
			.finish()
	}
}
