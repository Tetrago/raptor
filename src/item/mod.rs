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

	Const {
		Identifier("const"),
	}

	Qualifier {
		c: Optional::<Const<'a>>,
		Operator("*"),
	}

	Question {
		Operator("?"),
	}

	Exclamation {
		Operator("!"),
	}

	Type {
		segments: OneOrMore::<Segment<'a>, SegmentDelimiter, false>,
		quals: Vec::<Qualifier<'a>>,
		opt: Optional::<Question<'a>>,
		err: Optional::<Exclamation<'a>>,
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

	Unwrap {
		expr: UnaryExpression,
		Operator("!" | "?") as op,
	}

	PrimaryExpression {
		UnaryOperation | Invocation | Unwrap | UnaryExpression
	}

	GeometricOperation {
		lhs: PrimaryExpression,
		Operator("*" | "/") as op,
		rhs: Box::<SingularExpression<'a>>,
	}

	ArithmeticOperation {
		lhs: PrimaryExpression,
		Operator("+" | "-") as op,
		rhs: Box::<SingularExpression<'a>>,
	}

	Inequality {
		lhs: PrimaryExpression,
		Operator(">" | "<" | ">=" | "<=" | "==" | "!=") as op,
		rhs: Box::<SingularExpression<'a>>,
	}

	BooleanOperation {
		lhs: PrimaryExpression,
		Operator("||" | "&&") as op,
		rhs: Box::<SingularExpression<'a>>,
	}

	TernaryOperation {
		cond: PrimaryExpression,
		Operator("?"),
		lhs: Box::<SingularExpression<'a>>,
		rhs: Box::<SingularExpression<'a>>,
	}

	BinaryOperation {
		lhs: PrimaryExpression,
		op: Operator,
		rhs: Box::<SingularExpression<'a>>,
	}

	SingularExpression {
		GeometricOperation | ArithmeticOperation | Inequality | BooleanOperation | TernaryOperation | BinaryOperation | PrimaryExpression
	}

	Expression {
		Type
		// TODO: all
	}

	Empty {}
}
