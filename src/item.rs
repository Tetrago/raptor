use std::fmt;

use crate::group;
use crate::lexer::Parseable;
use crate::lexer::TokenStream;
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

		#[cfg_attr(coverage, coverage(off))]
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
	(@expand $obj:ident { $($field:ident),* $(,)? } $stream:ident ($(,)?) => $body:block) => {
		Some({ $body })
	};
	(@expand $obj:ident { $($field:ident),+ $(,)? } $stream:ident ($(,)?)) => {
		Some($obj {
			$($field: $field.into()),+
		})
	};
	(@expand $obj:ident { $($field:ident),* $(,)? } $stream:ident ($ident:ident ($($value:literal)|+) $(as $name:ident)?, $($item:tt)*) $(=> $body:block)?) => {
		match $stream.next() {
			#[allow(unused_parens)]
			$($name @)? ($(Some($crate::Spanned { value: Token::$ident($ident($value)), .. }))|+) => {
				$(
					let $name = match $name {
						Some($crate::Spanned { span, value: Token::$ident(value) }) => span.wrap(value),
						_ => unreachable!(),
					};
				)?

				parse!(@expand $obj { $($field,)* $($name)? } $stream ($($item)*) $(=> $body)?)
			}
			_ => None
		}
	};
	(@expand $obj:ident { $($field:ident),* $(,)? } $stream:ident ($t:ty as $name:ident, $($item:tt)*) $(=> $body:block)?) => {
		if let Some($name) = <$t>::parse($stream) {
			parse!(@expand $obj { $($field,)* $name } $stream ($($item)*) $(=> $body)?)
		} else {
			None
		}
	};
	($($ident:ident { $($item:tt)+ } $(=> $body:block)? ;)+) => {
		$(
			impl<'a> Parseable<'a> for $ident<'a> {
				type Target = Self;

				fn parse(stream: &mut TokenStream<'a>) -> Option<Self::Target> {
					stream.with(|stream| {
						parse!(@expand $ident {} stream ($($item)+,) $(=> $body)?)
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
				type Target = Self;

				fn parse(stream: &mut TokenStream<'a>) -> Option<Self::Target> {
					generic!(@or $ident stream $($name,)+)
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

	Break {}

	GeometricOperation {
		lhs: PrimaryExpression,
		op: {Operator},
		rhs: (Expression),
	}

	ArithmeticOperation {
		lhs: PrimaryExpression,
		op: {Operator},
		rhs: (Expression),
	}

	BinaryOperation {
		lhs: PrimaryExpression,
		op: {Operator},
		rhs: (Expression),
	}

	Group {
		expr: (Expression),
	}

	Invocation {
		expr: PrimaryExpression,
		args: [Expression],
	}

	MonoOperation {
		op: {Operator},
		expr: (Expression),
	}

	Evaluate {
		expr: Expression,
	}

	Let {
		ident: {Identifier},
		expr: Expression,
	}

	If {
		cond: Expression,
		stmt: (Statement),
	}

	While {
		cond: Expression,
		stmt: (Statement),
	}

	Until {
		cond: Expression,
		stmt: (Statement),
	}

	Do {
		cond: Expression,
		stmt: (Statement),
	}

	Loop {
		stmt: (Statement),
	}

	Return {
		expr: Expression,
	}

	For {
		init: (Statement),
		cond: Expression,
		eval: Expression,
		stmt: (Statement),
	}

	Field {
		name: {Identifier},
		ty: {Identifier},
	}

	Struct {
		name: {Identifier},
		fields: [Field],
	}

	Parameter {
		name: {Identifier},
		ty: {Identifier},
	}

	Function {
		name: {Identifier},
		params: [Parameter],
		stmt: Statement,
	}

	File {
		items: [Item],
	}
}

newtype! {
	#[derive(PartialEq, Eq, Clone)]
	pub type IdentifierExpression<'a> = Spanned<'a, Identifier<'a>>;

	#[derive(PartialEq, Eq, Clone)]
	pub type LiteralExpression<'a> = Spanned<'a, Literal<'a>>;
}

impl fmt::Debug for IdentifierExpression<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
		self.value.fmt(f)
	}
}

impl fmt::Debug for LiteralExpression<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
		self.value.fmt(f)
	}
}

generic! {
	PrimaryExpression {
		MonoOperation,
		Group,
		IdentifierExpression,
		LiteralExpression,
	}

	BinaryExpression {
		GeometricOperation,
		ArithmeticOperation,
		BinaryOperation,
	}

	Expression {
		BinaryExpression,
		Invocation,
		PrimaryExpression,
	}

	Statement {
		Block,
		Break,
		Do,
		Empty,
		Evaluate,
		For,
		If,
		Let,
		Loop,
		Return,
		Until,
		While,
	}

	Item {
		Function,
		Struct,
	}
}

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

				Some(items)
			})
			.or(Some(Vec::new()))
	}
}

parse! {
	IdentifierExpression { Identifier as ident } => { ident.into() };
	LiteralExpression { Literal as literal } => { literal.into() };

	Empty { Separator(";") } => {
		Empty::default()
	};

	Block {
		Separator("{"),
		List<Statement> as stmts,
		Separator("}"),
	};

	Break {
		Identifier("break"),
		Separator(";"),
	} => {
		Break::default()
	};

	GeometricOperation {
		PrimaryExpression as lhs,
		Operator("*" | "/") as op,
		Expression as rhs,
	};

	ArithmeticOperation {
		PrimaryExpression as lhs,
		Operator("+" | "-") as op,
		Expression as rhs,
	};

	BinaryOperation {
		PrimaryExpression as lhs,
		Operator as op,
		Expression as rhs,
	};

	Group {
		Separator("("),
		Expression as expr,
		Separator(")"),
	};

	Invocation {
		PrimaryExpression as expr,
		Separator("("),
		CommaList<Expression> as args,
		Separator(")"),
	};

	MonoOperation {
		Operator as op,
		Expression as expr,
	};

	Evaluate {
		Expression as expr,
		Separator(";"),
	};

	Let {
		Identifier("let"),
		Identifier as ident,
		Operator("="),
		Expression as expr,
		Separator(";"),
	};

	If {
		Identifier("if"),
		Separator("("),
		Expression as cond,
		Separator(")"),
		Statement as stmt,
	};

	While {
		Identifier("while"),
		Separator("("),
		Expression as cond,
		Separator(")"),
		Statement as stmt,
	};

	Until {
		Identifier("until"),
		Separator("("),
		Expression as cond,
		Separator(")"),
		Statement as stmt,
	};

	Do {
		Identifier("do"),
		Statement as stmt,
		Identifier("while"),
		Separator("("),
		Expression as cond,
		Separator(")"),
		Separator(";"),
	};

	Return {
		Identifier("return"),
		Expression as expr,
		Separator(";"),
	};

	Loop {
		Identifier("loop"),
		Statement as stmt,
	};

	For {
		Identifier("for"),
		Separator("("),
		Statement as init,
		Expression as cond,
		Separator(";"),
		Expression as eval,
		Separator(")"),
		Statement as stmt,
	};

	Field {
		Identifier as name,
		Operator(":"),
		Identifier as ty,
	};

	Struct {
		Identifier("struct"),
		Identifier as name,
		Separator("{"),
		CommaList<Field> as fields,
		Separator("}"),
	};

	Parameter {
		Identifier as name,
		Operator(":"),
		Identifier as ty,
	};

	Function {
		Identifier("fn"),
		Identifier as name,
		Separator("("),
		CommaList<Parameter> as params,
		Separator(")"),
		Statement as stmt,
	};
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
