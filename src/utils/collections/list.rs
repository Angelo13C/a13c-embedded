extern crate alloc;
use alloc::vec::Vec;
use core::{ops::Deref, slice::SliceIndex};

pub trait List<T>
{
	fn length(&self) -> usize;
	fn capacity(&self) -> usize;
	fn extend(&mut self, items: &[T]) -> Result<(), WouldOverflow>;
	fn push(&mut self, item: T) -> Result<(), WouldOverflow>;
	fn swap_remove(&mut self, index: usize) -> Option<T>;
	fn clear(&mut self);
	fn get(&self, index: usize) -> Option<&T>;
	fn get_mut(&mut self, index: usize) -> Option<&mut T>;
	fn get_slice<I: SliceIndex<[T], Output = [T]>>(&self, index: I) -> &[T];
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
pub struct WouldOverflow;

pub struct ArrayList<const N: usize, T>
{
	array: [T; N],
	length: usize,
}

impl<const N: usize, T> List<T> for ArrayList<N, T>
where T: Clone + Copy
{
	fn capacity(&self) -> usize
	{
		N
	}

	fn length(&self) -> usize
	{
		self.length
	}

	fn extend(&mut self, items: &[T]) -> Result<(), WouldOverflow>
	{
		if self.length + items.len() >= N
		{
			Err(WouldOverflow)
		}
		else
		{
			for i in 0..items.len()
			{
				self.array[self.length + i] = items[i].clone();
			}
			self.length += items.len();

			Ok(())
		}
	}

	fn push(&mut self, item: T) -> Result<(), WouldOverflow>
	{
		if self.length + 1 == N
		{
			Err(WouldOverflow)
		}
		else
		{
			self.array[self.length] = item;
			self.length += 1;

			Ok(())
		}
	}

	fn swap_remove(&mut self, index: usize) -> Option<T>
	{
		(index < self.length).then(|| {
			let removed_element = self.array[index].clone();
			if index != self.length - 1
			{
				self.array[index] = self.array[self.length - 1].clone();
			}
			self.length -= 1;

			removed_element
		})
	}

	fn clear(&mut self)
	{
		self.length = 0;
	}

	fn get(&self, index: usize) -> Option<&T>
	{
		(index < self.length).then(|| &self.array[index])
	}

	fn get_mut(&mut self, index: usize) -> Option<&mut T>
	{
		(index < self.length).then(|| &mut self.array[index])
	}

	fn get_slice<I: SliceIndex<[T], Output = [T]>>(&self, index: I) -> &[T]
	{
		&self.array[index]
	}
}

impl<T> List<T> for Vec<T>
where T: Clone
{
	fn capacity(&self) -> usize
	{
		Vec::capacity(self)
	}

	fn length(&self) -> usize
	{
		Vec::len(self)
	}

	fn extend(&mut self, items: &[T]) -> Result<(), WouldOverflow>
	{
		Vec::extend_from_slice(self, items);

		Ok(())
	}

	fn push(&mut self, item: T) -> Result<(), WouldOverflow>
	{
		Vec::push(self, item);

		Ok(())
	}

	fn swap_remove(&mut self, index: usize) -> Option<T>
	{
		Some(Vec::swap_remove(self, index))
	}

	fn clear(&mut self)
	{
		Vec::clear(self)
	}

	fn get(&self, index: usize) -> Option<&T>
	{
		<Vec<T> as Deref>::Target::get(self, index)
	}

	fn get_mut(&mut self, index: usize) -> Option<&mut T>
	{
		<Vec<T> as Deref>::Target::get_mut(self, index)
	}

	fn get_slice<I: SliceIndex<[T], Output = [T]>>(&self, index: I) -> &[T]
	{
		&self[index]
	}
}
