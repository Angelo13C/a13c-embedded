use core::ops::{Add, Range, RangeBounds, RangeInclusive};

pub trait RangeExt<T: ?Sized>: RangeBounds<T>
{
	fn intersection(&self, other: &Self) -> Self;
	fn shift(&self, by: T) -> Self;
}

impl<T: Ord + Clone + Add<T, Output = T>> RangeExt<T> for Range<T>
{
	/// # Examples
	/// ```
	/// use a13c_embedded::utils::range::*;
	///
	/// let a = 0..20;
	/// let b = 10..30;
	/// assert_eq!(a.intersection(&b), 10..20);
	///
	/// let a = 0..10;
	/// let b = 15..30;
	/// assert!(a.intersection(&b).is_empty());
	/// ```
	fn intersection(&self, other: &Self) -> Self
	{
		core::cmp::max(self.start.clone(), other.start.clone())..core::cmp::min(self.end.clone(), other.end.clone())
	}

	/// # Examples
	/// ```
	/// use a13c_embedded::utils::range::*;
	///
	/// let range = 0..20;
	/// assert_eq!(range.shift(10), 10..30);
	///
	/// let range = 10..20;
	/// assert_eq!(range.shift(-10), 0..10);
	/// ```
	fn shift(&self, by: T) -> Self
	{
		(self.start.clone() + by.clone())..(self.end.clone() + by.clone())
	}
}

impl<T: Ord + Clone + Add<T, Output = T>> RangeExt<T> for RangeInclusive<T>
{
	/// # Examples
	/// ```
	/// use a13c_embedded::utils::range::*;
	///
	/// let a = 0..=20;
	/// let b = 10..=30;
	///
	/// assert_eq!(a.intersection(&b), 10..=20);
	///
	/// let a = 0..=10;
	/// let b = 15..=30;
	///
	/// assert!(a.intersection(&b).is_empty());
	/// ```
	fn intersection(&self, other: &Self) -> Self
	{
		core::cmp::max(self.start().clone(), other.start().clone())
			..=core::cmp::min(self.end().clone(), other.end().clone())
	}

	/// # Examples
	/// ```
	/// use a13c_embedded::utils::range::*;
	///
	/// let range = 0..=20;
	/// assert_eq!(range.shift(10), 10..=30);
	///
	/// let range = 10..=20;
	/// assert_eq!(range.shift(-10), 0..=10);
	/// ```
	fn shift(&self, by: T) -> Self
	{
		(self.start().clone() + by.clone())..=(self.end().clone() + by.clone())
	}
}
