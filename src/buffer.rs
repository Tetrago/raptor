use std::collections::VecDeque;

pub struct Buffered<'a, T: Clone> {
	iter: Box<dyn Iterator<Item = T> + 'a>,

	/// The items that still need to be read by buffered stacks.
	queue: VecDeque<T>,
	/// The list of next indices for all buffered stacks.
	stacks: Vec<usize>,
}

impl<'a, T: Clone> Buffered<'a, T> {
	pub fn new(iter: impl Iterator<Item = T> + 'a) -> Self {
		Self {
			iter: Box::new(iter),
			queue: VecDeque::new(),
			stacks: Vec::new(),
		}
	}

	/// Save the current location in the iterator.
	pub fn push(&mut self) {
		if let Some(i) = self.stacks.last() {
			self.stacks.push(*i);
		} else {
			self.stacks.push(0);
		}
	}

	/// Forget the last push location, saving the items for future retrieval.
	pub fn restore(&mut self) {
		self.stacks.pop();
	}

	/// Remove all items since the last push from future retrievals.
	pub fn pop(&mut self) {
		if self.stacks.pop().is_some() {
			if let Some(i) = self.stacks.last() {
				debug_assert!(
					*i <= self.queue.len(),
					"Expected stack index to be within queue, but index {i} exceedes length {}",
					self.queue.len()
				);

				self.queue.resize_with(*i, || unreachable!());
			} else {
				self.queue.clear();
			}
		}
	}

	/// Pushes to the buffer, pops when Some is returned and restores given
	/// None.
	pub fn with<R>(&mut self, f: impl FnOnce(&mut Self) -> Option<R>) -> Option<R> {
		self.push();
		let result = f(self);

		if result.is_some() {
			self.pop();
			result
		} else {
			self.restore();
			None
		}
	}
}

impl<'a, T: Clone> Iterator for Buffered<'a, T> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(i) = self.stacks.last_mut() {
			if let Some(item) = self.queue.get(*i) {
				// Return the item in the queue if we aren't at the front of it, where we need
				// to request a new item.

				*i += 1;
				Some(item.clone())
			} else if let Some(item) = self.iter.next() {
				self.queue.push_back(item.clone());
				*i += 1;
				Some(item)
			} else {
				None
			}
		} else if self.queue.is_empty() {
			self.iter.next()
		} else {
			debug_assert!(
				self.stacks.is_empty(),
				"Expected stacks to be empty beforing removing from the queue"
			);

			self.queue.pop_front()
		}
	}
}

pub trait BufferedExt<'a>: Iterator
where
	Self::Item: Clone,
{
	fn buffered(self) -> Buffered<'a, Self::Item>;
}

#[cfg_attr(coverage, coverage(off))]
impl<'a, T> BufferedExt<'a> for T
where
	T: Iterator + 'a,
	T::Item: Clone,
{
	fn buffered(self) -> Buffered<'a, Self::Item> {
		Buffered::new(Box::new(self))
	}
}
