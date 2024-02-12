//! Module for handling temperature measurements and conversions.
//!
//! This module provides a versatile [`Temperature`] struct for representing temperature values in various units,
//! along with methods for conversion between different temperature scales.
//!
//! # Examples
//! ```
//! use a13c_embedded::utils::physical_quantities::temperature::*;
//!
//! // Create a temperature of 20 degrees Celsius
//! let celsius_temp = Temperature::from_celsius(20.0);
//!
//! // Convert the temperature to Kelvin
//! let kelvin_temp = celsius_temp.as_kelvin();
//!
//! // Convert the temperature to Fahrenheit
//! let fahrenheit_temp = celsius_temp.as_fahrenheit();
//!
//! // Display the temperature in different units
//! println!("Temperature: {}", celsius_temp); // Output: Temperature: 20°C
//! println!("Temperature: {:?}", kelvin_temp); // Output: Temperature: 293.15 K
//! println!("Temperature: {:?}", fahrenheit_temp); // Output: Temperature: 68°F
//! ```

use core::{
	fmt::{Debug, Display},
	ops::{Add, Mul, Sub},
};

/// Equivalent of `0°C` in the Kelvin scale.
pub const ZERO_CELSIUS_IN_KELVIN: f32 = 273.15;

/// A temperature value.
#[derive(Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Temperature<T: TemperatureType = f32>
{
	kelvin: T,
}

impl<T: TemperatureType> Temperature<T>
{
	/// Constructs a [`Temperature`] from the provided degrees Celsius value.
	pub fn from_celsius(degrees: T) -> Self
	{
		Self::from_kelvin(degrees + T::from(ZERO_CELSIUS_IN_KELVIN))
	}

	/// Constructs a [`Temperature`] from the provided Kelvin value.
	pub const fn from_kelvin(kelvin: T) -> Self
	{
		Self { kelvin }
	}

	/// Constructs a `Temperature` from the provided Fahrenheit value.
	pub fn from_fahrenheit(degrees: T) -> Self
	{
		Self::from_kelvin((degrees - T::from(32.)) * T::from(1.8) + T::from(ZERO_CELSIUS_IN_KELVIN))
	}

	/// Returns the value of [`Self`] in degrees Celsius.
	///
	/// # Example
	/// ```
	/// # use a13c_embedded::utils::physical_quantities::temperature::*;
	/// #
	/// assert_eq!(Temperature::from_kelvin(0.).as_celsius(), -ZERO_CELSIUS_IN_KELVIN);
	/// assert_eq!(Temperature::from_celsius(0.).as_celsius(), 0.);
	/// ```
	pub fn as_celsius(&self) -> T
	{
		self.kelvin - T::from(ZERO_CELSIUS_IN_KELVIN)
	}

	/// Returns the value of [`Self`] in Kelvin.
	///
	/// # Example
	/// ```
	/// # use a13c_embedded::utils::physical_quantities::temperature::*;
	/// #
	/// assert_eq!(Temperature::from_kelvin(0.).as_kelvin(), 0.);
	/// assert_eq!(Temperature::from_celsius(0.).as_kelvin(), ZERO_CELSIUS_IN_KELVIN);
	/// ```
	pub const fn as_kelvin(&self) -> T
	{
		self.kelvin
	}

	/// Returns the value of [`Self`] in degrees Fahrenheit.
	///
	/// # Example
	/// ```
	/// # use a13c_embedded::utils::physical_quantities::temperature::*;
	/// #
	/// assert_eq!(Temperature::from_celsius(0.).as_fahrenheit(), 32.);
	/// // There is a really small precision errors here due to how floating point math works
	/// assert_eq!((Temperature::from_kelvin(1.).as_fahrenheit() * 100_f32).round() / 100., -457.87);
	/// ```
	pub fn as_fahrenheit(&self) -> T
	{
		T::from(1.8) * self.as_celsius() + T::from(32.)
	}
}

/// A trait representing a type that can be used to do calculations in a [`Temperature`] struct.
pub trait TemperatureType:
	Clone + Copy + From<f32> + Add<Self, Output = Self> + Sub<Self, Output = Self> + Mul<Self, Output = Self>
{
}
impl<T: Clone + Copy + From<f32> + Add<Self, Output = Self> + Sub<Self, Output = Self> + Mul<Self, Output = Self>>
	TemperatureType for T
{
}

impl Add<Self> for Temperature
{
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output
	{
		Self::from_kelvin(self.kelvin + rhs.kelvin)
	}
}
impl Sub<Self> for Temperature
{
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output
	{
		Self::from_kelvin(self.kelvin - rhs.kelvin)
	}
}

impl Display for Temperature
{
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
	{
		write!(f, "{}°C", self.as_celsius())
	}
}

impl Debug for Temperature
{
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
	{
		write!(f, "{} K", self.as_kelvin())
	}
}
