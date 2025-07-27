use crate::lexer::Operator;
use crate::lexer::Parseable;
use crate::lexer::Separator;
use crate::lexer::Token;
use crate::lexer::TokenStream;
use crate::Spanned;

pub struct CommaDelimiter;

impl<'a> Parseable<'a> for CommaDelimiter {
	type Target = ();

	fn parse(stream: &mut TokenStream<'a>) -> Option<Self::Target> {
		if let Some(Spanned {
			value: Token::Separator(Separator(",")),
			..
		}) = stream.next()
		{
			Some(())
		} else {
			None
		}
	}
}

pub struct SegmentDelimiter;

impl<'a> Parseable<'a> for SegmentDelimiter {
	type Target = ();

	fn parse(stream: &mut TokenStream<'a>) -> Option<Self::Target> {
		if let Some(Spanned {
			value: Token::Operator(Operator("::")),
			..
		}) = stream.next()
		{
			Some(())
		} else {
			None
		}
	}
}

pub struct List<Item, Delimiter, const TRAILING: bool, const REQUIRED: bool>(
	std::marker::PhantomData<Item>,
	std::marker::PhantomData<Delimiter>,
);

impl<
		'a,
		Item: Parseable<'a>,
		Delimiter: Parseable<'a>,
		const TRAILING: bool,
		const REQUIRED: bool,
	> Parseable<'a> for List<Item, Delimiter, TRAILING, REQUIRED>
{
	type Target = Vec<Item::Target>;

	fn parse(stream: &mut TokenStream<'a>) -> Option<Self::Target> {
		let mut items = Vec::new();

		if let Some(item) = Item::parse(stream) {
			items.push(item);

			// Collect all instances of delimiter followed by the item.
			items.extend(std::iter::from_fn(|| {
				stream.with(|stream| Delimiter::parse(stream).and_then(|_| Item::parse(stream)))
			}));

			if TRAILING {
				Delimiter::parse(stream);
			}
		} else if REQUIRED {
			return None;
		}

		Some(items)
	}
}

pub type OneOrMore<Item, Delimiter, const TRAILING: bool> = List<Item, Delimiter, TRAILING, true>;
pub type ZeroOrMore<Item, Delimiter, const TRAILING: bool> = List<Item, Delimiter, TRAILING, false>;

#[derive(Debug, PartialEq, Eq)]
pub struct Optional<T>(std::marker::PhantomData<T>);

impl<'a, T: Parseable<'a>> Parseable<'a> for Optional<T> {
	type Target = Option<T::Target>;

	fn parse(stream: &mut TokenStream<'a>) -> Option<Self::Target> {
		stream.with(|stream| T::parse(stream).map(Some).or(Some(None)))
	}
}

impl<'a, T: Parseable<'a>> Parseable<'a> for Box<T> {
	type Target = Box<T::Target>;

	fn parse(stream: &mut TokenStream<'a>) -> Option<Self::Target> {
		stream.with(T::parse).map(Box::new)
	}
}
