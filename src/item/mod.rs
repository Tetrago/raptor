mod dsl;
pub mod utility;

use utility::*;

use crate::lexer::*;
use crate::*;

item_dsl! {
	Generic {
		Separator("<"),
		types: ZeroOrMore::<Box<Type<'a>>, CommaDelimiter, false>,
		ty: Type,
		Separator(">"),
	}

	Segment {
		ident: Identifier,
		generic: Box<Optional<Generic<'a>>>,
	}

	Type {
		segments: OneOrMore::<Segment<'a>, SegmentDelimiter, false>,
		// TODO: qualifiers
		// TODO: suffix
	}

	Group {
		Separator("("),
		expr: Expression,
		Separator(")"),
	}

	UnaryExpression {
		Group | Identifier | Literal
	}

	UnaryOperation {
		op: Operator,
		expr: UnaryExpression,
	}

	Invocation {
		expr: UnaryExpression,
		generic: Optional::<Generic<'a>>,
		Separator("("),
		args: ZeroOrMore::<SingularExpression<'a>, CommaDelimiter, false>,
		Separator(")"),
	}

	SingularExpression {
		Type
		// TODO: all
	}

	Expression {
		Type
		// TODO: all
	}
}
