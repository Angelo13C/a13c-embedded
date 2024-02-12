//! Module for handling frequency measurements and conversions.
//!
//! This module provides a versatile [`Frequency`] struct for representing frequency values in various orders of magnitudes,
//! along with methods for conversion between different orders of magnitudes.
//!
//! # Examples
//!
//! ```
//! # use a13c_embedded::utils::physical_quantities::frequency::*;
//! #
//! let hertz = Frequency::from_hertz(1);
//! assert_eq!(hertz.as_hertz(), 1);
//!
//! let kilohertz = Frequency::from_kilohertz(1);
//! assert_eq!(kilohertz.as_hertz(), 1_000);
//!
//! let megahertz = Frequency::from_megahertz(1);
//! assert_eq!(megahertz.as_hertz(), 1_000_000);
//! ```

use core::ops::Div;

use super::duration::SmallDuration;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A frequency value with a `1Hz` sensitivity and a range of values that goes up to `2^32Hz` (which is almost 4.3GHz).
pub struct Frequency
{
	hertz: u32,
}

impl Frequency
{
	/// Represents a zero frequency (`0Hz`).
	pub const ZERO: Self = Frequency::from_hertz(0);

	/// Constructs a [`Frequency`] from the provided `hertz` value.
	///
	/// # Examples
	/// ```
	/// # use a13c_embedded::utils::physical_quantities::frequency::Frequency;
	/// #
	/// assert_eq!(Frequency::from_hertz(1).as_hertz(), 1);
	/// assert_eq!(Frequency::from_hertz(100).as_hertz(), 100);
	/// assert_eq!(Frequency::from_hertz(10_000).as_hertz(), 10_000);
	/// ```
	pub const fn from_hertz(hertz: u32) -> Self
	{
		Self { hertz }
	}

	/// Constructs a [`Frequency`] from the provided `kilohertz` value.
	///
	/// # Examples
	/// ```
	/// # use a13c_embedded::utils::physical_quantities::frequency::Frequency;
	/// #
	/// assert_eq!(Frequency::from_kilohertz_f32(1.).as_hertz(), 1_000);
	/// assert_eq!(Frequency::from_kilohertz_f32(4.9).as_hertz(), 4_900);
	/// assert_eq!(Frequency::from_kilohertz_f32(13.5).as_hertz(), 13_500);
	/// ```
	pub fn from_kilohertz_f32(kilohertz: f32) -> Self
	{
		Self::from_hertz((kilohertz * 1_000.) as u32)
	}

	/// Constructs a [`Frequency`] from the provided `kilohertz` value. (differently from [`Self::from_kilohertz`], this function is constant
	/// because it works with integers and not floating point values)
	///
	/// # Examples
	/// ```
	/// # use a13c_embedded::utils::physical_quantities::frequency::Frequency;
	/// #
	/// assert_eq!(Frequency::from_kilohertz(1).as_hertz(), 1_000);
	/// assert_eq!(Frequency::from_kilohertz(45).as_hertz(), 45_000);
	/// ```
	pub const fn from_kilohertz(kilohertz: u32) -> Self
	{
		Self::from_hertz(kilohertz * 1_000)
	}

	/// Constructs a [`Frequency`] from the provided `megahertz` value.
	///
	/// # Examples
	/// ```
	/// # use a13c_embedded::utils::physical_quantities::frequency::Frequency;
	/// #
	/// assert_eq!(Frequency::from_megahertz_f32(1.).as_kilohertz(), 1_000.);
	/// assert_eq!(Frequency::from_megahertz_f32(4.9).as_kilohertz(), 4_900.);
	/// assert_eq!(Frequency::from_megahertz_f32(13.5).as_kilohertz(), 13_500.);
	/// ```
	pub fn from_megahertz_f32(megahertz: f32) -> Self
	{
		Self::from_kilohertz_f32(megahertz * 1_000.)
	}

	/// Constructs a [`Frequency`] from the provided `megahertz` value. (differently from [`Self::from_megahertz`], this function is constant
	/// because it works with integers and not floating point values)
	///
	/// # Examples
	/// ```
	/// # use a13c_embedded::utils::physical_quantities::frequency::Frequency;
	/// #
	/// assert_eq!(Frequency::from_megahertz(1).as_hertz(), 1_000_000);
	/// assert_eq!(Frequency::from_megahertz(45).as_hertz(), 45_000_000);
	/// ```
	pub const fn from_megahertz(megahertz: u32) -> Self
	{
		Self::from_kilohertz(megahertz * 1_000)
	}

	/// Returns the value of this frequency in hertz (`Hz`).
	pub const fn as_hertz(&self) -> u32
	{
		self.hertz
	}

	/// Returns the value of this frequency in kilohertz (`KHz`).
	pub fn as_kilohertz(&self) -> f32
	{
		self.hertz as f32 / 1_000.
	}

	/// Returns the value of this frequency in megahertz (`MHz`).
	pub fn as_megahertz(&self) -> f32
	{
		self.as_kilohertz() / 1_000.
	}
}

impl From<SmallDuration> for Frequency
{
	/// Converts the provided [`SmallDuration`] into a [`Frequency`].
	///
	/// ```
	/// # use a13c_embedded::utils::physical_quantities::{frequency::Frequency, duration::SmallDuration};
	/// #
	/// assert_eq!(Into::<Frequency>::into(SmallDuration::from_millis(50)), Frequency::from_hertz(20));
	/// assert_eq!(Into::<Frequency>::into(SmallDuration::from_micros(1)), Frequency::from_hertz(1_000_000));
	/// ```
	fn from(value: SmallDuration) -> Self
	{
		Self::from_hertz(SmallDuration::SECOND.as_tens_of_nanos() / value.as_tens_of_nanos())
	}
}

impl Div<u32> for Frequency
{
	type Output = Self;

	fn div(self, rhs: u32) -> Self::Output
	{
		Self::from_hertz(self.as_hertz() / rhs)
	}
}
