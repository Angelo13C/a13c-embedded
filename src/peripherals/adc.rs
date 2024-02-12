//! [`Analog-to-Digital Converter`] (ADC) peripherals.
//!
//! ## Usage
//!
//! To use the ADC functionality, implement the [`Adc`] trait for your ADC peripheral and the [`AdcPin`] trait
//! for your ADC pin type. Then, you can use the provided methods to read ADC values.
//!
//! ## Example
//!
//! ```rust
//! use a13c_embedded::peripherals::adc::{Adc, AdcPin, AdcPinExt};
//! use a13c_embedded::utils::math::Percentage;
//!
//! // Use AdcPinExt trait to read ADC values as percentages
//! let mut adc = MyAdc;
//! let mut adc_pin = MyAdcPin;
//! match adc_pin.read_percentage(&mut adc) {
//!     Ok(percentage) => println!("ADC percentage: {:?}", percentage),
//!     Err(err) => println!("Error reading ADC: {:?}", err),
//! }
//!
//! // This is how you could implement an ADC peripheral for your specific hardware
//!
//! pub struct AdcReading(u16);
//! impl core::ops::Div<AdcReading> for AdcReading
//! {
//!     type Output = Result<Percentage, ()>;
//!
//!     fn div(self, rhs: AdcReading) -> Self::Output
//!     {
//!          Percentage::from_0_to_1(self.0 as f32 / rhs.0 as f32).map_err(|_| ())
//!     }
//! }
//!
//! // Implement Adc trait for your ADC peripheral
//! struct MyAdc;
//! impl Adc for MyAdc {
//!     type ReadableValue = AdcReading;
//!     fn max_readable_value(&self) -> Self::ReadableValue {
//!         AdcReading(4095) // Example: Maximum ADC value for 12-bit ADC
//!     }
//! }
//!
//! // Implement AdcPin trait for your ADC pin type
//! struct MyAdcPin;
//! impl AdcPin<MyAdc> for MyAdcPin {
//!     type Error = ();
//!     fn read(&mut self, adc: &mut MyAdc) -> Result<AdcReading, Self::Error> {
//!         let result = 10; // Perform ADC conversion instead of using a static 10 value, and return the result...
//!
//!         Ok(AdcReading(result))
//!     }
//! }
//! ```
//!
//! [`Analog-to-Digital Converter`]: <https://en.wikipedia.org/wiki/Analog-to-digital_converter>

use core::{fmt::Debug, ops::Div};

use crate::utils::math::Percentage;

/// A trait for [`ADC`] peripherals.
/// It defines methods for obtaining the maximum readable value and performing ADC conversions.
///
/// Check [`module's`] documentation for an example usage.
///
/// [`ADC`]: <https://en.wikipedia.org/wiki/Analog-to-digital_converter>
/// [`module's`]: self
pub trait Adc
{
	/// The type representing a readable `ADC` value.
	type ReadableValue: Div<Self::ReadableValue, Output = Result<Percentage, ()>>;

	/// Returns the maximum readable value of this `ADC`.
	fn max_readable_value(&self) -> Self::ReadableValue;
}

/// A trait for `ADC` pins. It defines methods for reading [`Adc`] values from a pin.
pub trait AdcPin<A: Adc>
{
	/// The type representing an error encountered while reading an `ADC` value.
	type Error: Debug;

	/// Reads the `ADC` value from the pin.
	fn read(&mut self, adc: &mut A) -> Result<A::ReadableValue, Self::Error>;
}

/// An extension trait for [`ADC pins`], providing additional functionality for reading `ADC` values as percentages.
///
/// [`ADC pins`]: AdcPin
pub trait AdcPinExt<A: Adc>: AdcPin<A>
{
	/// Reads the `ADC` value as a [`Percentage`].
	fn read_percentage(&mut self, adc: &mut A) -> Result<Percentage, ReadPercentageError<A, Self>>
	where Self: Sized;
}

impl<P: AdcPin<A>, A: Adc> AdcPinExt<A> for P
{
	fn read_percentage(&mut self, adc: &mut A) -> Result<Percentage, ReadPercentageError<A, Self>>
	where Self: Sized
	{
		let read = self.read(adc).map_err(|err| ReadPercentageError::CantRead(err))?;
		(read / adc.max_readable_value()).map_err(|_| ReadPercentageError::InvalidPercentage)
	}
}

/// An enum representing errors that can occur when [`reading ADC values as percentages`].
///
/// [`reading ADC values as percentages`]: AdcPinExt::read_percentage
pub enum ReadPercentageError<A: Adc, P: AdcPin<A>>
{
	/// An error indicating that the `ADC` value could not be read.
	CantRead(P::Error),
	/// An error indicating that the calculated percentage is invalid.
	InvalidPercentage,
}

impl<A: Adc, P: AdcPin<A>> Debug for ReadPercentageError<A, P>
{
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
	{
		match self
		{
			Self::CantRead(arg0) => f.debug_tuple("Can't read").field(arg0).finish(),
			Self::InvalidPercentage => write!(f, "Invalid percentage"),
		}
	}
}
