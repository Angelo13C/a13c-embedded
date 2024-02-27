use core::ops::RangeInclusive;

use crate::utils::physical_quantities::temperature::Temperature;

/// Makes sure a [`Temperature`] is in the allowed range you passed when constructing an instance of this
/// struct.
///
/// # Examples
/// ```
/// # use a13c_embedded::{features::temperature::safety::allowed_range::*, utils::physical_quantities::temperature::Temperature};
/// #
/// let allowed_temperature_range_safety = AllowedTemperatureRangeSafety::new(
///      Temperature::from_celsius(0.)..=Temperature::from_celsius(250.));
///
/// assert!(allowed_temperature_range_safety.is_temperature_safe(Temperature::from_celsius(100.)));
/// assert!(allowed_temperature_range_safety.is_temperature_safe(Temperature::from_celsius(250.)));
/// assert!(allowed_temperature_range_safety.is_temperature_safe(Temperature::from_celsius(0.)));
///
/// assert!(!allowed_temperature_range_safety.is_temperature_safe(Temperature::from_celsius(-20.)));
/// assert!(!allowed_temperature_range_safety.is_temperature_safe(Temperature::from_celsius(300.)));
/// assert!(!allowed_temperature_range_safety.is_temperature_safe(Temperature::from_celsius(-1.)));
/// ```
pub struct AllowedTemperatureRangeSafety
{
	allowed_range: RangeInclusive<Temperature>,
}

impl AllowedTemperatureRangeSafety
{
	/// Check [`struct's documentation`](Self).
	pub const fn new(allowed_range: RangeInclusive<Temperature>) -> Self
	{
		Self { allowed_range }
	}

	/// Check [`struct's documentation`](Self).
	pub fn is_temperature_safe(&self, temperature: Temperature) -> bool
	{
		self.allowed_range.contains(&temperature)
	}
}