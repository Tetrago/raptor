use crate::buffer::Buffered;
use crate::group;
use crate::newtype;
use crate::prelude::*;
use crate::token::*;

macro_rules! item {
	(@expand $name:ident) => { $name<'a> };
	(@expand ($name:tt)) => { Box<item!(@expand $name)> };
	(@expand [$name:tt]) => { Vec<item!(@expand $name)> };
	(@item $name:ident {}) => {
		#[derive(Default, Clone, Eq, PartialEq)]
		pub struct $name<'a> {
			_marker: ::std::marker::PhantomData<&'a ()>
		}

		impl ::std::fmt::Debug for $name<'_> {
			fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::result::Result<(), ::std::fmt::Error> {
				write!(f, stringify!($name))
			}
		}
	};
	(@item $name:ident { $($field:ident : $tt:tt),* $(,)? }) => {
		#[derive(Debug, Clone, Eq, PartialEq)]
		pub struct $name<'a> {
			$(pub $field: item!(@expand $tt)),*
		}
	};
	($(
		$name:ident {
			$($field:ident : $tt:tt),* $(,)?
		}
	)*) => {
		$(item!(@item $name { $($field : $tt),* });)*
	};
}

macro_rules! parse {
	(@expand $stream:ident () => $body:block) => {
		Some({ $body })
	};
	(@expand $stream:ident ($ident:ident ($name:ident), $($item:tt)*) => $body:block) => {
		if let Some((_, $crate::token::Token::$ident($name))) = $stream.next() {
			parse!(@expand $stream ($($item)*) => $body)
		} else {
			None
		}
	};
	(@expand $stream:ident ($ident:ident ($value:literal), $($item:tt)*) => $body:block) => {
		if let Some((_, $crate::token::Token::$ident($ident($value)))) = $stream.next() {
			parse!(@expand $stream ($($item)*) => $body)
		} else {
			None
		}
	};
	(@expand $stream:ident ($t:ty as $name:ident, $($item:tt)*) => $body:block) => {
		if let Some($name) = <$t>::parse($stream) {
			parse!(@expand $stream ($($item)*) => $body)
		} else {
			None
		}
	};
	($($ident:ident ($($item:tt)+) => $body:block),+ $(,)?) => {
		$(
			impl<'a> Parseable<'a> for $ident<'a> {
				fn parse(stream: &mut TokenStream<'a>) -> Option<Self> {
					stream.with(|stream| {
						parse!(@expand stream ($($item)+,) => $body)
					})
				}
			}
		)+
	};
}

macro_rules! generic {
	(@or $ident:ident $stream:ident) => {
		None
	};
	(@or $ident:ident $stream:ident $item:ident, $($rest:ident,)*) => {
		$item::parse($stream).map($ident::$item).or_else(|| {
			generic!(@or $ident $stream $($rest,)*)
		})
	};
	($(
		$ident:ident {
			$($name:ident),+ $(,)?
		}
	)+) => {
		$(
			impl<'a> Parseable<'a> for $ident<'a> {
				fn parse(stream: &mut TokenStream<'a>) -> Option<Self> {
					generic!(@or $ident stream $($name,)+)
				}
			}

			impl ::std::fmt::Debug for $ident<'_> {
				fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::result::Result<(), ::std::fmt::Error> {
					match self {
						$($ident::$name(x) => write!(f, "{:?}", x)),+
					}
				}
			}
		)+

		group! {
			$(
				#[derive(Clone, Eq, PartialEq)]
				pub enum $ident {
					$($name),+
				}
			)+
		}
	};
}

item! {
	Empty {}

	Block {
		stmts: [Statement]
	}

	BinaryOperation {
		lhs: (Expression),
		op: Operator,
		rhs: (Expression),
	}

	Group {
		expr: (Expression),
	}

	Invocation {
		expr: (Expression),
		args: [Expression],
	}

	MonoOperation {
		op: Operator,
		expr: (Expression),
	}

	Let {
		ident: Identifier,
		expr: Expression
	}

	If {
		cond: Expression,
		stmt: (Statement)
	}

	While {
		cond: Expression,
		stmt: (Statement)
	}

	Until {
		cond: Expression,
		stmt: (Statement)
	}

	Do {
		cond: Expression,
		stmt: (Statement)
	}

	Loop {
		stmt: (Statement)
	}

	For {
		init: (Statement),
		cond: Expression,
		eval: Expression,
		stmt: (Statement)
	}

	Field {
		name: Identifier,
		ty: Identifier
	}

	Struct {
		name: Identifier,
		fields: [Field]
	}

	Parameter {
		name: Identifier,
		ty: Identifier
	}

	Function {
		name: Identifier,
		params: [Parameter],
		stmt: Statement
	}

	File {
		items: [Item]
	}
}

impl<'a> Parseable<'a> for Literal<'a> {
	fn parse(stream: &mut TokenStream<'a>) -> Option<Self> {
		stream.with(|stream| {
			stream.next().and_then(|(_, token)| match token {
				Token::Literal(literal) => Some(literal),
				_ => None,
			})
		})
	}
}

impl<'a> Parseable<'a> for Identifier<'a> {
	fn parse(stream: &mut TokenStream<'a>) -> Option<Self> {
		stream.with(|stream| {
			stream.next().and_then(|(_, token)| match token {
				Token::Identifier(ident) => Some(ident),
				_ => None,
			})
		})
	}
}

generic! {
	Expression {
		MonoOperation,
		BinaryOperation,
		Invocation,
		Group,
		Identifier,
		Literal,
	}

	Statement {
		Empty,
		Block,
		Do,
		For,
		If,
		Let,
		Loop,
		While,
		Until,
	}

	Item {
		Function,
		Struct,
	}
}

type TokenStream<'a> = Buffered<'a, (Span<'a>, Token<'a>)>;

trait Parseable<'a>
where
	Self: Sized + 'a,
{
	fn parse(stream: &mut TokenStream<'a>) -> Option<Self>;
}

newtype! {
	type List<T> = Vec<T>;
	type CommaList<T> = Vec<T>;
}

impl<'a, T: Parseable<'a>> Parseable<'a> for List<T> {
	fn parse(stream: &mut TokenStream<'a>) -> Option<Self> {
		Some(
			std::iter::from_fn(|| stream.with(T::parse))
				.collect::<Vec<_>>()
				.into(),
		)
	}
}

impl<'a, T: Parseable<'a>> Parseable<'a> for CommaList<T> {
	fn parse(stream: &mut TokenStream<'a>) -> Option<Self> {
		stream
			.with(|stream| {
				let mut items = vec![T::parse(stream)?];

				while stream
					.with(|stream| {
						stream.next().map(|(_, token)| {
							if matches!(token, Token::Operator(Operator(","))) {
								Some(())
							} else {
								None
							}
						})
					})
					.is_some()
				{
					if let Some(item) = stream.with(T::parse) {
						items.push(item)
					} else {
						break;
					}
				}

				Some(items.into())
			})
			.or(Some(Vec::new().into()))
	}
}

parse! {
	Empty(Separator(";")) => {
		Empty::default()
	},

	Block(List<Statement> as stmts) => {
		Block {
			stmts: stmts.into(),
		}
	},

	BinaryOperation(Expression as lhs, Operator(op), Expression as rhs) => {
		BinaryOperation {
			lhs: Box::new(lhs),
			op,
			rhs: Box::new(rhs),
		}
	},

	Group(Separator("("), Expression as expr, Separator(")")) => {
		Group {
			expr: Box::new(expr),
		}
	},

	Invocation(Expression as expr, CommaList<Expression> as args) => {
		Invocation {
			expr: Box::new(expr),
			args: args.into(),
		}
	},

	MonoOperation(Operator(op), Expression as expr) => {
		MonoOperation {
			op,
			expr: Box::new(expr),
		}
	},

	Let(Identifier(ident), Expression as expr) => {
		Let {
			ident,
			expr
		}
	},

	If(Expression as cond, Statement as stmt) => {
		If {
			cond,
			stmt: Box::new(stmt),
		}
	},

	While(Expression as cond, Statement as stmt) => {
		While {
			cond,
			stmt: Box::new(stmt),
		}
	},

	Until(Expression as cond, Statement as stmt) => {
		Until {
			cond,
			stmt: Box::new(stmt),
		}
	},

	Do(Expression as cond, Statement as stmt) => {
		Do {
			cond,
			stmt: Box::new(stmt),
		}
	},

	Loop(Statement as stmt) => {
		Loop {
			stmt: Box::new(stmt),
		}
	},

	For(Statement as init, Expression as cond, Expression as eval, Statement as stmt) => {
		For {
			init: Box::new(init),
			cond,
			eval,
			stmt: Box::new(stmt),
		}
	},

	Field(Identifier(name), Operator(":"), Identifier(ty)) => {
		Field { name, ty }
	},

	Struct(Identifier(name), Separator("{"), CommaList<Field> as fields, Separator("}")) => {
		Struct {
			name,
			fields: fields.into(),
		}
	},

	Parameter(Identifier(name), Operator(":"), Identifier(ty)) => {
		Parameter { name, ty }
	},

	Function(Identifier("fn"), Identifier(name), Separator("("), CommaList<Parameter> as params, Separator(")"), Statement as stmt) => {
		Function {
			name,
			params: params.into(),
			stmt,
		}
	}
}

impl File<'_> {
	pub fn parse<'a>(lexer: Lexer<'a>) -> File<'a> {
		let mut stream = lexer
			.filter(|(_, token)| !matches!(token, Token::Whitespace(_) | Token::Comment(_)))
			.buffered();

		let items = std::iter::from_fn(|| Item::parse(&mut stream)).collect();

		File { items }
	}
}
