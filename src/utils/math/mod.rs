//! Module containing mathematical utility functions.
//!
//! This module provides a collection of utility functions for common mathematical operations,
//! such as mapping values between ranges, linear interpolation, and value constraint within a range.
//!
//! # Examples
//!
//! ```
//! use a13c_embedded::utils::math::*;
//!
//! // Map a value from one range to another
//! let mapped_value = map(0.5, 0.0..=1.0, 10.0..=20.0);
//! println!("Mapped value: {}", mapped_value);  // Output: Mapped value: 15
//! # assert_eq!(format!("Mapped value: {}", mapped_value), "Mapped value: 15");
//!
//! // Linearly interpolate between start and end values of a range
//! let interpolated_value = lerp(0.5, 0.0..=100.0);
//! println!("Interpolated value: {}", interpolated_value);  // Output: Interpolated value: 50
//! # assert_eq!(format!("Interpolated value: {}", interpolated_value), "Interpolated value: 50");
//!
//! // Constrain a value within a specified range
//! let constrained_value = constrain(30, 0..=20);
//! println!("Contrained value: {}", constrained_value);  // Output: Contrained value: 20
//! # assert_eq!(format!("Contrained value: {}", constrained_value), "Contrained value: 20");
//! ```

mod percentage;

use core::ops::*;

pub use percentage::*;

/// Maps a value from one range to another.
///
/// Given a value `value` within the range `from`, this function maps it to a corresponding value
/// within the range `to` using [`linear interpolation`].
///
/// # Examples
///
/// ```
/// # use a13c_embedded::utils::math::map;
/// #
/// let mapped_value = map(0.5, 0.0..=1.0, 10.0..=20.0);
/// assert_eq!(mapped_value, 15.0);
/// ```
///
/// [`linear interpolation`]: <https://en.wikipedia.org/wiki/Linear_interpolation>
pub fn map<T>(value: T, from: RangeInclusive<T>, to: RangeInclusive<T>) -> T
where
	T: Clone + Copy + Sub<T, Output = T> + Mul<T, Output = T> + Add<T, Output = T> + Div<T, Output = T>,
{
	(value - *from.start()) * (*to.end() - *to.start()) / (*from.end() - *from.start()) + *to.start()
}

/// [`Linearly interpolates`] between `range.start()` and `range.end()`.
///
/// # Examples
/// ```
/// # use a13c_embedded::utils::math;
/// #
/// assert_eq!(math::lerp(0.0, 0_f32..=100.), 0.);
/// assert_eq!(math::lerp(0.5, 0_f32..=100.), 50.);
/// assert_eq!(math::lerp(1.0, 0_f32..=100.), 100.);
/// ```
///
/// [`Linearly interpolates`]: <https://en.wikipedia.org/wiki/Linear_interpolation>
pub fn lerp<T>(t: f32, range: RangeInclusive<T>) -> T
where
	T: Clone + Copy + Sub<T, Output = T> + Mul<f32, Output = T> + Add<T, Output = T>,
{
	*range.start() + (*range.end() - *range.start()) * t
}

/// Returns `value` if `value` is contained in the provided `range`, `range.start()` if `value` is smaller
/// than `range.start()` and `range.end()` if `value` is greater than `range.end()`.
///
/// # Examples
/// ```
/// # use a13c_embedded::utils::math::constrain;
/// #
/// assert_eq!(constrain(10, 0..=20), 10);
/// assert_eq!(constrain(-5, 0..=20), 0);
/// assert_eq!(constrain(40, 0..=20), 20);
/// ```
pub fn constrain<T>(value: T, range: RangeInclusive<T>) -> T
where
	T: Copy + PartialOrd,
{
	if value < *range.start() {
		*range.start()
	} else if value > *range.end() {
		*range.end()
	} else {
		value
	}
}
