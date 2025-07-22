use crate::lexer::Parseable;
use crate::lexer::Separator;
use crate::lexer::Token;
use crate::lexer::TokenStream;
use crate::Spanned;

pub struct List<T>(std::marker::PhantomData<T>);
pub struct CommaList<T>(std::marker::PhantomData<T>);

impl<'a, T: Parseable<'a>> Parseable<'a> for List<T> {
	type Target = Vec<T::Target>;

	fn parse(stream: &mut TokenStream<'a>) -> Option<Self::Target> {
		Some(std::iter::from_fn(|| stream.with(T::parse)).collect())
	}
}

impl<'a, T: Parseable<'a>> Parseable<'a> for CommaList<T> {
	type Target = Vec<T::Target>;

	fn parse(stream: &mut TokenStream<'a>) -> Option<Self::Target> {
		stream
			.with(|stream| {
				let mut items = vec![T::parse(stream)?];

				while stream
					.with(|stream| {
						stream.next().and_then(|Spanned { value, .. }| match value {
							Token::Separator(Separator(",")) => Some(()),
							_ => None,
						})
					})
					.is_some()
				{
					if let Some(item) = stream.with(T::parse) {
						items.push(item);
					} else {
						break;
					}
				}

				Some(items)
			})
			.or(Some(Vec::new()))
	}
}

pub struct Optional<T>(std::marker::PhantomData<T>);

impl<'a, T: Parseable<'a>> Parseable<'a> for Optional<T> {
	type Target = Option<T::Target>;

	fn parse(stream: &mut TokenStream<'a>) -> Option<Self::Target> {
		T::parse(stream).map(Option::Some)
	}
}
