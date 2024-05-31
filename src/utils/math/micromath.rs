//! This module re-exports the `micromath` crate and adds a bit of functionality to its content.

pub use micromath;
use micromath::vector::Vector2d;

use super::shapes::Component;

pub trait VectorExt<C: Component>
{
	fn length(&self) -> C;
	fn length_squared(&self) -> C;
}

impl<C: Component> VectorExt<C> for Vector2d<C>
{
	fn length(&self) -> C
	{
		self.length_squared().sqrt()
	}

	fn length_squared(&self) -> C
	{
		self.x * self.x + self.y * self.y
	}
}
