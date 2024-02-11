//! Module for common utility functions and helpers.
//!
//! This module provides a collection of utility functions and helpers that are commonly used
//! across different parts of the project. These utilities aim to simplify common tasks and
//! promote code reuse and readability.

/// Clamps the provided `value` within the specified range.
///
/// If `value` is lower than the `min`, the `min` is returned.
/// If `value` is higher than the `max`, the `max` is returned.
/// Otherwise, the original `value` is returned.
///
/// # Examples
/// ```
/// # use a13c_embedded::utils::*;
///
/// assert_eq!(clamp(5, 0, 10), 5);
/// assert_eq!(clamp(12, 0, 10), 10);
/// assert_eq!(clamp(-3, 0, 10), 0);
/// ```
pub fn clamp<T: Ord>(value: T, min: T, max: T) -> T {
	if value < min {
		min
	} else if value > max {
		max
	} else {
		value
	}
}
