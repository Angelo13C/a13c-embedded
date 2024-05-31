extern crate alloc;

use alloc::sync::Arc;
use core::{
	convert::Infallible,
	sync::atomic::{AtomicU32, Ordering},
};

use embedded_hal::digital::{OutputPin, StatefulOutputPin};

use super::PwmPin;
use crate::{
	peripherals::time::timer::*,
	utils::{math::Percentage, physical_quantities::duration::SmallDuration},
};

pub struct TimerPwm<T: Timer>
{
	timer: T,
	ticks_count_while_on: Arc<AtomicU32>,
	ticks_count_while_off: Arc<AtomicU32>,
}

impl<T: Timer> TimerPwm<T>
{
	/// Returns a [`TimerPwm`] that can control its heat percentage through the provided `pin`.
	pub fn new(
		mut pin: impl StatefulOutputPin + Send + 'static, mut timer: T, single_cycle_duration: SmallDuration,
	) -> Result<Self, T::Error>
	{
		let ticks_count_while_on = Arc::new(AtomicU32::new(0));
		let ticks_count_while_on_cloned = Arc::clone(&ticks_count_while_on);
		let ticks_count_while_off = Arc::new(AtomicU32::new(0));
		let ticks_count_while_off_cloned = Arc::clone(&ticks_count_while_off);

		let clock_frequency = timer.get_clock_frequency();
		let mut timer_internal = timer.get_additional_functionality();

		timer.enable_alarm(true)?;
		unsafe {
			timer.on_alarm(move || {
				const ORDERING: Ordering = Ordering::Relaxed;

				let duration = match pin.is_set_high().unwrap()
				{
					true =>
					{
						pin.set_low().unwrap();
						ticks_count_while_on_cloned.load(ORDERING)
					},
					false =>
					{
						pin.set_high().unwrap();
						ticks_count_while_off_cloned.load(ORDERING)
					},
				};

				let alarm = timer_internal.get_time_in_ticks().unwrap()
					+ super::timer::small_duration_to_counter(
						clock_frequency,
						SmallDuration::from_tens_of_nanos(duration),
					);
				timer_internal.set_alarm_in_ticks(alarm).unwrap();
			})?;
		}

		Ok(Self {
			timer,
			ticks_count_while_on,
			ticks_count_while_off,
		})
	}
}

impl<T: Timer> PwmPin for TimerPwm<T>
{
	type Error = Infallible;

	fn get_duty_cycle(&self) -> Percentage
	{
		todo!()
	}

	fn set_duty_cycle(&mut self, percentage: Percentage) -> Result<(), Self::Error>
	{
		todo!()
	}

	fn set_frequency(
		&mut self, frequency: crate::utils::physical_quantities::frequency::Frequency,
	) -> Result<(), Self::Error>
	{
		todo!()
	}
}
