macro_rules! token {
	($($name:ident => $pat:literal),+ $(,)?) => {
		$(
			#[derive(Debug, PartialEq, Eq, Clone, Copy)]
			pub struct $name<'a>(pub &'a str);

			impl<'a> $name<'a> {
				fn regex() -> &'static ::regex::Regex {
					use ::regex::Regex;
					use ::std::sync::LazyLock;

					static INSTANCE: LazyLock<Regex> = LazyLock::new(|| Regex::new($pat).unwrap());
					&*INSTANCE
				}
			}
		)+

		$crate::group! {
			#[derive(Debug, PartialEq, Eq, Clone, Copy)]
			pub enum Token {
				$($name),+
			}
		}

		impl<'a> Token<'a> {
			pub fn slice(&self) -> &'a str {
				match self {
					$(
						Self::$name(x) => x.0
					),*
				}
			}
		}

		impl<'a> Iterator for Lexer<'a> {
			type Item = (Span<'a>, Token<'a>);

			fn next(&mut self) -> Option<Self::Item> {
				$(
					if let Some(span) = $name::regex()
						.find(&self.contents[self.index..])
						.map(|m| m.end())
						.map(|len| self.take(len))
					{
						return Some((span, $name(span.slice()).into()));
					}
				)+;

				None
			}
		}
	};
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Span<'a> {
	pub src: &'a str,
	pub from: usize,
	pub to: usize,
}

impl<'a> Span<'a> {
	pub fn slice(&self) -> &'a str {
		&self.src[self.from..self.to]
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

	fn take(&mut self, count: usize) -> Span<'a> {
		assert!(self.index + count <= self.contents.len());

		let span = Span {
			src: self.contents,
			from: self.index,
			to: self.index + count,
		};

		self.index += count;
		span
	}
}

token! {
	Comment => r"^//.*(\n|$)|^/\*.*\*/",
	Literal => r"^\d*\.\d+|^(0[xbo])?\d+",
	Identifier => r"^[A-Za-z]\w*|^`[^`]+`",
	Separator => r"^[\[\]\(\){};,]",
	Whitespace => r"^\s+",
	Operator => r"^[^\w\s\[\]\(\){};]+",
}
