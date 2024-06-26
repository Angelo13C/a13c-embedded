use core::{
	fmt::{Debug, Display},
	ops::{Add, AddAssign, Sub, SubAssign},
};

/// A percentage value.
#[derive(PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct Percentage(f32);

impl Percentage
{
	/// The `0%` value.
	pub const ZERO: Self = Self(0.);
	/// The `50%` value.
	pub const HALF: Self = Self(0.5);
	/// The `100%` value.
	pub const FULL: Self = Self(1.);

	/// Creates a new [`Percentage`] from the provided value that must be in the range `0..=100`.
	///
	/// Returns `Ok(Percentage)` if the condition above is met, otherwise returns `Err(())`.
	///
	/// # Example
	/// ```
	/// # use a13c_embedded::utils::math::Percentage;
	/// #
	/// assert!(Percentage::from_0_to_100(50.).is_ok());
	/// assert!(Percentage::from_0_to_100(250.).is_err());
	/// assert!(Percentage::from_0_to_100(-10.).is_err());
	/// ```
	pub fn from_0_to_100(value: f32) -> Result<Self, ValueOutOfRange>
	{
		Self::from_0_to_1(value / 100.)
	}

	/// Creates a new [`Percentage`] from the provided value that must be in the range `0..=1`.
	///
	/// Returns `Ok(Percentage)` if the condition above is met, otherwise returns `Err(())`.
	///
	/// # Example
	/// ```
	/// # use a13c_embedded::utils::math::Percentage;
	/// #
	/// assert!(Percentage::from_0_to_1(0.5).is_ok());
	/// assert!(Percentage::from_0_to_1(2.5).is_err());
	/// assert!(Percentage::from_0_to_1(-0.2).is_err());
	/// ```
	pub fn from_0_to_1(value: f32) -> Result<Self, ValueOutOfRange>
	{
		if (0_f32..=1_f32).contains(&value)
		{
			Ok(Self(value))
		}
		else
		{
			Err(ValueOutOfRange)
		}
	}

	/// Returns the value of the percentage in the range `0..=1`.
	///
	/// # Example
	/// ```
	/// # use a13c_embedded::utils::math::Percentage;
	/// #
	/// assert_eq!(Percentage::from_0_to_1(1.).unwrap().into_0_to_1(), 1.);
	/// assert_eq!(Percentage::from_0_to_100(30.).unwrap().into_0_to_1(), 0.3);
	/// ```
	pub fn into_0_to_1(&self) -> f32
	{
		self.0
	}

	/// Returns the value of the percentage in the range `0..=100`.
	///
	/// # Example
	/// ```
	/// # use a13c_embedded::utils::math::Percentage;
	/// #
	/// assert_eq!(Percentage::from_0_to_100(100.).unwrap().into_0_to_100(), 100.);
	/// assert_eq!(Percentage::from_0_to_1(0.8).unwrap().into_0_to_100(), 80.);
	/// ```
	pub fn into_0_to_100(&self) -> f32
	{
		self.into_0_to_1() * 100.
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ValueOutOfRange;

impl Debug for Percentage
{
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
	{
		write!(f, "{}%", self.0 * 100.)
	}
}

impl Display for Percentage
{
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
	{
		write!(f, "{}%", self.0 * 100.)
	}
}

impl Add for Percentage
{
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output
	{
		Self::from_0_to_1(self.into_0_to_1() + rhs.into_0_to_1()).unwrap_or(Percentage::FULL)
	}
}

impl AddAssign for Percentage
{
	fn add_assign(&mut self, rhs: Self)
	{
		*self = *self + rhs;
	}
}

impl Sub for Percentage
{
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output
	{
		Self::from_0_to_1(self.into_0_to_1() - rhs.into_0_to_1()).unwrap_or_else(|_| {
			if self.into_0_to_1() > rhs.into_0_to_1()
			{
				Percentage::FULL
			}
			else
			{
				Percentage::ZERO
			}
		})
	}
}

impl SubAssign for Percentage
{
	fn sub_assign(&mut self, rhs: Self)
	{
		*self = *self - rhs;
	}
}
