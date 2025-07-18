use crate::buffer::Buffered;
use crate::group;
use crate::lexer::*;
use crate::newtype;
use crate::prelude::*;
use crate::Spanned;

macro_rules! item {
	(@expand {$name:tt}) => { $crate::Spanned<'a, item!(@expand $name)> };
	(@expand ($name:tt)) => { Box<item!(@expand $name)> };
	(@expand [$name:tt]) => { Vec<item!(@expand $name)> };
	(@expand $name:ident) => { $name<'a> };
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
		if let Some(Spanned { span, value: Token::$ident($name) }) = $stream.next() {
			let $name = span.wrap($name);
			parse!(@expand $stream ($($item)*) => $body)
		} else {
			None
		}
	};
	(@expand $stream:ident ($ident:ident ($value:literal), $($item:tt)*) => $body:block) => {
		if let Some(Spanned { value: Token::$ident($ident($value)), .. }) = $stream.next() {
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
		op: {Operator},
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
		op: {Operator},
		expr: (Expression),
	}

	Let {
		ident: {Identifier},
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
		name: {Identifier},
		ty: {Identifier}
	}

	Struct {
		name: {Identifier},
		fields: [Field]
	}

	Parameter {
		name: {Identifier},
		ty: {Identifier}
	}

	Function {
		name: {Identifier},
		params: [Parameter],
		stmt: Statement
	}

	File {
		items: [Item]
	}
}

newtype! {
	#[derive(Debug, PartialEq, Eq, Clone)]
	pub type IdentifierExpression<'a> = Spanned<'a, Identifier<'a>>;

	#[derive(Debug, PartialEq, Eq, Clone)]
	pub type LiteralExpression<'a> = Spanned<'a, Literal<'a>>;
}

impl<'a> Parseable<'a> for LiteralExpression<'a> {
	fn parse(stream: &mut TokenStream<'a>) -> Option<Self> {
		stream.with(|stream| {
			stream.next().and_then(|token| match token {
				Spanned {
					span,
					value: Token::Literal(literal),
				} => Some(span.wrap(literal).into()),
				_ => None,
			})
		})
	}
}

impl<'a> Parseable<'a> for IdentifierExpression<'a> {
	fn parse(stream: &mut TokenStream<'a>) -> Option<Self> {
		stream.with(|stream| {
			stream.next().and_then(|token| match token {
				Spanned {
					span,
					value: Token::Identifier(ident),
				} => Some(span.wrap(ident).into()),
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
		IdentifierExpression,
		LiteralExpression,
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

type TokenStream<'a> = Buffered<'a, Spanned<'a, Token<'a>>>;

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
						stream.next().and_then(|Spanned { value, .. }| match value {
							Token::Operator(Operator(",")) => Some(()),
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
			lhs: lhs.into(),
			op,
			rhs: rhs.into(),
		}
	},

	Group(Separator("("), Expression as expr, Separator(")")) => {
		Group {
			expr: expr.into(),
		}
	},

	Invocation(Expression as expr, CommaList<Expression> as args) => {
		Invocation {
			expr: expr.into(),
			args: args.into(),
		}
	},

	MonoOperation(Operator(op), Expression as expr) => {
		MonoOperation {
			op,
			expr: expr.into(),
		}
	},

	Let(Identifier(ident), Expression as expr) => {
		Let {
			ident,
			expr,
		}
	},

	If(Expression as cond, Statement as stmt) => {
		If {
			cond,
			stmt: stmt.into(),
		}
	},

	While(Expression as cond, Statement as stmt) => {
		While {
			cond,
			stmt: stmt.into(),
		}
	},

	Until(Expression as cond, Statement as stmt) => {
		Until {
			cond,
			stmt: stmt.into(),
		}
	},

	Do(Expression as cond, Statement as stmt) => {
		Do {
			cond,
			stmt: stmt.into(),
		}
	},

	Loop(Statement as stmt) => {
		Loop {
			stmt: stmt.into(),
		}
	},

	For(Statement as init, Expression as cond, Expression as eval, Statement as stmt) => {
		For {
			init: init.into(),
			cond,
			eval,
			stmt: stmt.into(),
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
			.filter(|spanned| !matches!(spanned.as_ref(), Token::Whitespace(_) | Token::Comment(_)))
			.buffered();

		let items = std::iter::from_fn(|| Item::parse(&mut stream)).collect();

		File { items }
	}
}
