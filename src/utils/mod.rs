//! Module for common utility functions and helpers.
//!
//! This module provides a collection of utility functions and helpers that are commonly used
//! across different parts of the project. These utilities aim to simplify common tasks and
//! promote code reuse and readability.

pub mod algorithms;
pub mod collections;
pub mod math;
pub mod physical_quantities;
pub mod range;

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
where T: Clone + Copy
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
where T: Clone + Copy
{
	core::array::from_fn(|i| *slice.get(i).unwrap_or(&fill))
}

/// Returns the digits count of `number`.
pub fn itoa(mut number: i32, string: &mut [u8]) -> u8
{
	let mut i = 0;
	loop
	{
		let digit = (number % 10) as u8;
		number /= 10;
		string[i] = digit + b'0';

		i += 1;

		if number == 0
		{
			// Swap the order of bytes
			for j in 0..(i / 2)
			{
				let temp = string[j];
				string[j] = string[i - j - 1];
				string[i - j - 1] = temp;
			}

			return i as u8;
		}
	}
}
