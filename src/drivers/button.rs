use embedded_hal::digital::{ErrorType, InputPin};

use crate::peripherals::interrupt::{InterruptPin, Trigger};

/// A button connected to the microcontroller that can read if it's pressed through the `P` pin.
pub struct Button<P: InputPin>
{
	pin: P,
}

impl<P: InputPin> Button<P>
{
	/// Returns a [`Button`] that can read its state on the provided `pin`.
	pub fn new(pin: P) -> Self
	{
		Self { pin }
	}

	/// Returns `Ok(true)` if the button is pressed, `Ok(false)` if it isn't pressed and
	/// `Err(<P as InputPin>::Error)` if there has been an error while reading the button's state.
	pub fn is_pressed(&mut self) -> Result<bool, <P as ErrorType>::Error>
	{
		self.pin.is_high()
	}
}

impl<P: InputPin + InterruptPin> Button<P>
{
	/// Call the provided `callback` function when the button is pressed (using an interrupt).
	///
	/// # Safety
	/// Check [`InterruptPin::subscribe_to_interrupt`].
	pub unsafe fn on_pressed(
		&mut self, callback: impl FnMut() + Send + 'static,
	) -> Result<(), <P as InterruptPin>::Error>
	{
		self.subscribe_to_interrupt(Trigger::PositiveEdge, callback)
	}
}

impl<P: InputPin> ErrorType for Button<P>
{
	type Error = P::Error;
}

impl<P: InputPin> InputPin for Button<P>
{
	fn is_high(&mut self) -> Result<bool, <Self as ErrorType>::Error>
	{
		self.pin.is_high()
	}

	fn is_low(&mut self) -> Result<bool, <Self as ErrorType>::Error>
	{
		self.pin.is_low()
	}
}

impl<P: InputPin + InterruptPin> InterruptPin for Button<P>
{
	type Error = <P as InterruptPin>::Error;

	/// # Safety
	/// Check [`InterruptPin::subscribe_to_interrupt`].
	unsafe fn subscribe_to_interrupt(
		&mut self, when_to_trigger: Trigger, callback: impl FnMut() + Send + 'static,
	) -> Result<(), Self::Error>
	{
		self.pin.subscribe_to_interrupt(when_to_trigger, callback)
	}
}
