//! The [`Bresenham's line algorithm`] is a line drawing algorithm that determines the points of an N-dimensional
//! raster that should be selected in order to form a close approximation to a straight line between two points.
//!
//! [`Bresenham's line algorithm`]: <https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm>

/// Implementation of [`Bresenham's line algorithm`](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm).
///
/// # Examples
/// ```
/// # use a13c_embedded::utils::algorithms::bresenham::Bresenham;
/// #
/// let mut bresenham = Bresenham::new([0, 0], [8, 3]);
///
/// assert_eq!(bresenham.next(), Some([true, false]));
/// assert_eq!(bresenham.next(), Some([true, true]));
/// assert_eq!(bresenham.next(), Some([true, false]));
/// assert_eq!(bresenham.next(), Some([true, false]));
/// assert_eq!(bresenham.next(), Some([true, true]));
/// assert_eq!(bresenham.next(), Some([true, false]));
/// assert_eq!(bresenham.next(), Some([true, true]));
/// assert_eq!(bresenham.next(), Some([true, false]));
/// assert_eq!(bresenham.next(), None);
///
/// assert_eq!(bresenham.get_longest_index(), 0);
/// ```
#[derive(Debug)]
pub struct Bresenham<const N: usize>
{
	signs: [i8; N],
	errors: [i32; N],
	lengths: [i32; N],
	longest_index: usize,
	count: i32,
	current_position: [i32; N],
}

impl<const N: usize> Bresenham<N>
{
	/// Check [`struct's documentation`](`Self`).
	pub fn new(start: [i32; N], end: [i32; N]) -> Self
	{
		assert!(N >= 2);

		let mut deltas = [0; N];
		let mut signs = [0; N];
		let mut lengths = [0; N];
		let mut longest_index = 0;
		for i in 0..N
		{
			deltas[i] = end[i] - start[i];
			lengths[i] = deltas[i].abs();
			signs[i] = deltas[i].signum() as i8;

			if lengths[i] > lengths[longest_index]
			{
				longest_index = i;
			}
		}

		let longest = lengths[longest_index];

		let errors = [longest / 2; N];

		Self {
			lengths,
			longest_index,
			count: longest,
			current_position: start,
			signs,
			errors,
		}
	}

	fn has_finished(&self) -> bool
	{
		self.count <= 0
	}

	pub fn steps_taken(&self) -> u32
	{
		(self.lengths[self.longest_index] - self.count) as u32
	}

	pub fn lengths(&self) -> i32
	{
		self.lengths[self.longest_index]
	}
	pub fn countt(&self) -> i32
	{
		self.count
	}

	/// Returns the index of the axis with the biggest distance to travel.
	///
	/// # Examples
	/// ```
	/// # use a13c_embedded::utils::algorithms::bresenham::Bresenham;
	/// #
	/// // 12 is the biggest number and it's at index 1 of the array
	/// assert_eq!(Bresenham::new([0, 0, 0], [4, 12, 7]).get_longest_index(), 1);
	/// // 24 is the biggest number and it's at index 0 of the array
	/// assert_eq!(Bresenham::new([0, 0, 0], [24, 12, 7]).get_longest_index(), 0);
	/// // 27 is the biggest number and it's at index 2 of the array
	/// assert_eq!(Bresenham::new([0, 0, 0], [4, 12, 27]).get_longest_index(), 2);
	/// ```
	pub fn get_longest_index(&self) -> usize
	{
		self.longest_index
	}
}

impl<const N: usize> Iterator for Bresenham<N>
{
	type Item = [bool; N];

	fn next(&mut self) -> Option<Self::Item>
	{
		(!self.has_finished()).then(|| {
			self.count -= 1;
			let mut changed_axis = [false; N];

			for i in 0..N
			{
				self.errors[i] -= self.lengths[i];
				if self.errors[i] < 0
				{
					self.errors[i] += self.lengths[self.longest_index];
					self.current_position[i] += self.signs[i] as i32;
					changed_axis[i] = true;
				}
			}

			changed_axis
		})
	}
}

impl<const N: usize> ExactSizeIterator for Bresenham<N>
{
	fn len(&self) -> usize
	{
		self.count as usize
	}
}
