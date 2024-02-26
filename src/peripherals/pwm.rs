//! Module for handling [`Pulse Width Modulation`] (PWM) pins.
//!
//! # Usage
//! This module provides a trait [`PwmPin`] for working with PWM pins.
//! It includes methods to get and set the duty cycle of the PWM signal, as well as to set the frequency.
//!
//! # Example
//! ```rust
//! use a13c_embedded::peripherals::pwm::*;
//! use a13c_embedded::utils::{math::Percentage, physical_quantities::frequency::Frequency};
//!
//! let mut pwm_pin = MyPwmPin;
//! # /*
//! // Set 100% duty cycle
//! pwm_pin.set_duty_cycle(Percentage::FULL).unwrap();
//! # */
//!
//! struct MyPwmPin;
//! impl PwmPin for MyPwmPin {
//!     type Error = ();
//!
//!     fn get_duty_cycle(&self) -> Percentage {
//!         // ...
//!         unimplemented!();
//!     }
//!
//!     fn set_duty_cycle(&mut self, percentage: Percentage) -> Result<(), Self::Error> {
//!         // ...
//!         unimplemented!();
//!     }
//!
//!     fn set_frequency(&mut self, frequency: Frequency) -> Result<(), Self::Error> {
//!         // ...
//!         unimplemented!();
//!     }
//! }
//! ```
//!
//! [`Pulse Width Modulation`]: <https://en.wikipedia.org/wiki/Pulse-width_modulation>

use core::fmt::Debug;

use crate::utils::{math::Percentage, physical_quantities::frequency::Frequency};

pub trait PwmPin
{
	/// Error type that can be returned by PWM operations.
	type Error: Debug;

	/// Get the current duty cycle of the PWM pin.
	fn get_duty_cycle(&self) -> Percentage;
	/// Set the duty cycle of the PWM pin.
	fn set_duty_cycle(&mut self, percentage: Percentage) -> Result<(), Self::Error>;

	/// Set the frequency of the PWM signal.
	fn set_frequency(&mut self, frequency: Frequency) -> Result<(), Self::Error>;
}
