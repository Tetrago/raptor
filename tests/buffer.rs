use raptor::prelude::*;

struct Generator(u32);

impl Iterator for Generator {
	type Item = u32;

	fn next(&mut self) -> Option<Self::Item> {
		let item = self.0;
		self.0 = item + 1;
		Some(item)
	}
}

struct Empty;

impl Iterator for Empty {
	type Item = u32;

	fn next(&mut self) -> Option<Self::Item> {
		None
	}
}

macro_rules! expect {
	($gen:ident => [ $($value:literal),* $(,)? ]) => {
		$(assert_eq!($gen.next(), Some($value)));*
	}
}

#[test]
fn basic() {
	let mut generator = Generator(0).buffered();
	expect!(generator => [0, 1, 2, 3, 4]);
}

#[test]
fn double_pop() {
	let mut generator = Generator(0).buffered();
	generator.pop();
}

#[test]
fn push_and_restore() {
	let mut generator = Generator(0).buffered();

	generator.push();
	expect!(generator => [0, 1, 2, 3, 4]);

	generator.restore();
	expect!(generator => [0, 1, 2, 3, 4]);
}

#[test]
fn push_and_pop() {
	let mut generator = Generator(0).buffered();

	generator.push();
	expect!(generator => [0, 1, 2, 3, 4]);

	generator.pop();
	expect!(generator => [5, 6, 7, 8, 9]);
}

#[test]
fn empty_push_and_pop() {
	let mut generator = Empty.buffered();

	generator.push();
	assert_eq!(generator.next(), None);
}

#[test]
fn nested_push_and_restore() {
	let mut generator = Generator(0).buffered();

	generator.push();
	expect!(generator => [0, 1, 2, 3, 4]);

	generator.push();
	expect!(generator => [5, 6, 7, 8, 9]);

	generator.restore();
	expect!(generator => [5, 6, 7, 8, 9]);
}

#[test]
fn nested_push_and_pop() {
	let mut generator = Generator(0).buffered();

	generator.push();
	expect!(generator => [0, 1, 2, 3, 4]);

	generator.push();
	expect!(generator => [5, 6, 7, 8, 9]);

	generator.pop();
	expect!(generator => [10, 11, 12, 13, 14]);
}

#[test]
fn with_none() {
	let mut generator = Generator(0).buffered();

	generator.with(|generator| {
		expect!(generator => [0, 1, 2, 3, 4]);
		Option::<()>::None
	});

	expect!(generator => [0, 1, 2, 3, 4]);
}

#[test]
fn with_some() {
	let mut generator = Generator(0).buffered();

	generator.with(|generator| {
		expect!(generator => [0, 1, 2, 3, 4]);
		Some(())
	});

	expect!(generator => [5, 6, 7, 8, 9]);
}
