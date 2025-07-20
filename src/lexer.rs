#[macro_export]
macro_rules! make_lexer {
	($($name:ident => $pat:literal),+ $(,)?) => {
		$(
			#[derive(PartialEq, Eq, Clone, Copy)]
			pub struct $name<'a>(pub &'a str);

			impl<'a> $name<'a> {
				fn regex() -> &'static ::regex::Regex {
					use ::regex::Regex;
					use ::std::sync::LazyLock;

					static INSTANCE: LazyLock<Regex> = LazyLock::new(|| Regex::new($pat).unwrap());
					&*INSTANCE
				}
			}

			impl ::std::fmt::Debug for $name<'_> {
				fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
					write!(f, concat!(stringify!($name), "(\"{}\")"), self.0)
				}
			}

			impl<'a> Parseable<'a> for $name<'a> {
				type Target = $crate::Spanned<'a, Self>;

				fn parse(stream: &mut TokenStream<'a>) -> Option<Self::Target> {
					stream.with(|stream| {
						match stream.next() {
							Some($crate::Spanned { span, value: Token::$name(token) }) => Some(span.wrap(token)),
							_ => None,
						}
					})
				}
			}
		)+

		$crate::group! {
			#[derive(PartialEq, Eq, Clone, Copy)]
			pub enum Token {
				$($name),+
			}
		}

		impl<'a> Token<'a> {
			pub fn as_str(&self) -> &'a str {
				match self {
					$(
						Self::$name(x) => x.0
					),*
				}
			}
		}

		impl<'a> Iterator for Lexer<'a> {
			type Item = $crate::Spanned<'a, Token<'a>>;

			fn next(&mut self) -> Option<Self::Item> {
				$(
					if let Some(span) = $name::regex()
						.find(&self.contents[self.index..])
						.map(|m| m.end())
						.map(|len| self.take(len))
					{
						return Some(span.wrap($name(span.as_str()).into()))
					}
				)+;

				None
			}
		}

		pub struct Lexer<'a> {
			contents: &'a str,
			index: usize,
		}

		impl<'a> Lexer<'a> {
			pub fn new(contents: &'a str) -> Self {
				Self { contents, index: 0 }
			}

			fn take(&mut self, count: usize) -> $crate::Span<'a> {
				use $crate::prelude::*;

				assert!(self.index + count <= self.contents.len());

				self.index += count;
				self.contents.span(self.index - count..self.index)
			}
		}

		pub type TokenStream<'a> = $crate::buffer::Buffered<'a, $crate::Spanned<'a, Token<'a>>>;

		pub trait Parseable<'a>
		where
			Self: Sized + 'a,
		{
			type Target;

			fn parse(stream: &mut TokenStream<'a>) -> Option<Self::Target>;
		}
	};
}

make_lexer! {
	Comment => r"^//.*(\n|$)|^/\*.*\*/",
	Literal => r"^\d*\.\d+|^(0[xbo])?\d+|^true|^false",
	Identifier => r"^[A-Za-z]\w*|^`[^`]+`",
	Separator => r"^[\[\]\(\){};]",
	Whitespace => r"^\s+",
	Operator => r"^[^\w\s\[\]\(\){};]+",
}
