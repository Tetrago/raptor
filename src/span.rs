use std::fmt;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::Range;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Span<'a> {
	pub source: &'a str,
	pub from: usize,
	pub to: usize,
}

impl<'a> Span<'a> {
	pub fn range(&self) -> Range<usize> {
		self.from..self.to
	}

	pub fn as_str(&self) -> &'a str {
		&self.source[self.from..self.to]
	}

	pub fn wrap<T>(&self, value: T) -> Spanned<'a, T> {
		Spanned { span: *self, value }
	}

	pub fn extend(&mut self, other: &Span<'_>) {
		self.from = self.from.min(other.from);
		self.to = self.to.min(other.to);
	}
}

pub trait SpanExt<'a> {
	fn span(&self, range: Range<usize>) -> Span<'a>;
}

impl<'a> SpanExt<'a> for &'a str {
	fn span(&self, range: Range<usize>) -> Span<'a> {
		Span {
			source: self,
			from: range.start,
			to: range.end,
		}
	}
}

pub struct Spanned<'a, T> {
	pub span: Span<'a>,
	pub value: T,
}

impl<T: Copy> Copy for Spanned<'_, T> {}

impl<T: Clone> Clone for Spanned<'_, T> {
	fn clone(&self) -> Self {
		self.span.wrap(self.value.clone())
	}
}

impl<T: fmt::Debug> fmt::Debug for Spanned<'_, T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_tuple("Spanned")
			.field(&(self.span.from..self.span.to))
			.field(&self.value)
			.finish()
	}
}

impl<T: PartialEq> PartialEq for Spanned<'_, T> {
	fn eq(&self, other: &Self) -> bool {
		self.span == other.span && self.value == other.value
	}
}

impl<T: Eq> Eq for Spanned<'_, T> {}

impl<T: PartialEq> PartialEq<T> for Spanned<'_, T> {
	fn eq(&self, other: &T) -> bool {
		self.value == *other
	}
}

impl<T> AsRef<T> for Spanned<'_, T> {
	fn as_ref(&self) -> &T {
		&self.value
	}
}

impl<T> AsMut<T> for Spanned<'_, T> {
	fn as_mut(&mut self) -> &mut T {
		&mut self.value
	}
}

impl<T> Deref for Spanned<'_, T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.value
	}
}

impl<T> DerefMut for Spanned<'_, T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.value
	}
}
