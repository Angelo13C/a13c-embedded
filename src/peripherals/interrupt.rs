//! Module for handling [`interrupt`] pins.
//!
//! # Usage
//! This module provides traits and types for working with pins capable of generating interrupts.
//! It includes the [`InterruptPin`] trait for subscribing to interrupts and the [`Trigger`] enum
//! representing various trigger conditions for interrupts.
//!
//! # Example
//!
//! ```rust
//! use a13c_embedded::peripherals::interrupt::{InterruptPin, Trigger};
//!
//! let mut pin = MyInterruptPin;
//! let callback = || {
//!     println!("Interrupt occurred!");
//! };
//!
//! unsafe {
//!     // Subscribe to an interrupt on the rising edge
//!     pin.subscribe_to_interrupt(Trigger::PositiveEdge, callback)
//!         .expect("Failed to subscribe to interrupt");
//! }
//!
//! // This is how you could implement an interrupt pin for your specific hardware
//!
//! struct MyInterruptPin;
//! impl InterruptPin for MyInterruptPin {
//!     type Error = ();
//!
//!     unsafe fn subscribe_to_interrupt(
//!         &mut self,
//!         when_to_trigger: Trigger,
//!         callback: impl FnMut() + Send + 'static,
//!     ) -> Result<(), Self::Error> {
//!         # return Ok(());
//!         unimplemented!();
//!     }
//! }
//! ```
//!
//! [`interrupt`]: <https://en.wikipedia.org/wiki/Interrupt>

use core::fmt::Debug;

/// A trait for pins capable of generating interrupts.
pub trait InterruptPin
{
	/// The type of error that may occur when subscribing to interrupts.
	type Error: Debug;

	/// Subscribes to interrupts generated by this pin.
	///
	/// # Safety
	/// The `callback` will be called in an ISR context.
	unsafe fn subscribe_to_interrupt(
		&mut self, when_to_trigger: Trigger, callback: impl FnMut() + Send + 'static,
	) -> Result<(), Self::Error>;
}

/// Enumeration of possible trigger conditions for interrupts.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Trigger
{
	/// Trigger on the positive edge of the signal.
	PositiveEdge,
	/// Trigger on the negative edge of the signal.
	NegativeEdge,
	/// Trigger on any edge (rising or falling).
	AnyEdge,
	/// Trigger on a low level signal.
	LowLevel,
	/// Trigger on a high level signal.
	HighLevel,
}