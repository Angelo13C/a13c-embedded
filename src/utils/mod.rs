//! Module for common utility functions and helpers.
//!
//! This module provides a collection of utility functions and helpers that are commonly used
//! across different parts of the project. These utilities aim to simplify common tasks and
//! promote code reuse and readability.

pub mod physical_quantities;

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

/// Converts the provided `slice` to an array of the same type and with the length `N`.
///
/// # Panics
/// Panics if the length of the slice is less than `N`.
///
/// # Examples
/// ```
/// # use a13c_embedded::utils::*;
/// #
/// let array = [5, 10, 20];
/// assert_eq!(slice_to_array(&array), array);
/// ```
///
/// This instead will panic because the array has a length of `3` which is less than `4`:
/// ```should_panic
/// # use a13c_embedded::utils::*;
/// #
/// let array = [5, 10, 20];
/// slice_to_array::<_, 4>(&array);
/// ```
pub fn slice_to_array<T, const N: usize>(slice: &[T]) -> [T; N]
where
	T: Clone + Copy,
{
	core::array::from_fn(|i| slice[i])
}

/// Converts the provided `slice` to an array of the same type and with the length `N`.
///
/// If the length of the slice is less than `N`, the remaining elements will be set to the
/// value of `fill`.
///
/// # Examples
/// ```
/// # use a13c_embedded::utils::*;
/// #
/// let array = [5, 10, 20];
/// assert_eq!(slice_to_array_filled(&array, -1), [5, 10, 20, -1, -1]);
/// ```
pub fn slice_to_array_filled<T, const N: usize>(slice: &[T], fill: T) -> [T; N]
where
	T: Clone + Copy,
{
	core::array::from_fn(|i| *slice.get(i).unwrap_or(&fill))
}
