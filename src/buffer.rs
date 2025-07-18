use std::collections::VecDeque;
use std::fmt;

pub struct Buffered<'a, T: Clone> {
	iter: Box<dyn Iterator<Item = T> + 'a>,

	/// The items that still need to be read by buffered stacks.
	queue: VecDeque<T>,

	/// The list of next indices for all buffered stacks.
	stacks: Vec<usize>,
}

impl<T: Clone + fmt::Debug> fmt::Debug for Buffered<'_, T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Buffered")
			.field("queue", &self.queue)
			.field("stacks", &self.stacks)
			.finish_non_exhaustive()
	}
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
		self.stacks.push(self.stacks.last().cloned().unwrap_or(0));
	}

	/// Forget the last push location, saving the items for future retrieval.
	pub fn restore(&mut self) {
		println!("pop");
		self.stacks.pop();
	}

	/// Move the next stack level forward, or clear the queue if no stacks are
	/// left.
	pub fn pop(&mut self) {
		if let Some(last) = self.stacks.pop() {
			if let Some(next) = self.stacks.last_mut() {
				*next = last;
			} else if last == self.queue.len() {
				self.queue.clear();
			} else {
				self.queue.drain(..last);
			}
		}
	}

	/// Pushes to the buffer, pops when Some is returned and restores given
	/// None.
	pub fn with<R>(&mut self, f: impl FnOnce(&mut Self) -> Option<R>) -> Option<R> {
		self.push();
		f(self).inspect(|_| self.pop()).or_else(|| {
			self.restore();
			None
		})
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
