use crate::group;
use crate::lexer::Parseable;
use crate::lexer::TokenStream;
use crate::lexer::*;
use crate::prelude::*;

mod macros;
pub mod utility;

use macros::*;
use utility::*;

item! {
	IdentifierExpression {
		ident: {Identifier}
	}

	LiteralExpression {
		literal: {Literal}
	}

	Empty {}

	Block {
		stmts: [Statement]
	}

	Break {}

	GeometricOperation {
		lhs: PrimaryExpression,
		op: {Operator},
		rhs: (GenericExpression),
	}

	ArithmeticOperation {
		lhs: PrimaryExpression,
		op: {Operator},
		rhs: (GenericExpression),
	}

	BinaryOperation {
		lhs: PrimaryExpression,
		op: {Operator},
		rhs: (GenericExpression),
	}

	Join {
		lhs: GenericExpression,
		rhs: (Expression),
	}

	Group {
		expr: (Expression),
	}

	Invocation {
		expr: UnaryExpression,
		args: [GenericExpression],
	}

	UnaryOperation {
		op: {Operator},
		expr: UnaryExpression,
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

generic! {
	UnaryExpression {
		Group,
		IdentifierExpression,
		LiteralExpression,
	}

	PrimaryExpression {
		UnaryOperation,
		Invocation,
		UnaryExpression,
	}

	GenericExpression {
		GeometricOperation,
		ArithmeticOperation,
		BinaryOperation,
		PrimaryExpression,
	}

	Expression {
		Join,
		GenericExpression,
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

parse! {
	IdentifierExpression { Identifier as ident };
	LiteralExpression { Literal as literal };
	Empty { Separator(";") };

	Block {
		Separator("{"),
		List<Statement> as stmts,
		Separator("}"),
	};

	Break {
		Identifier("break"),
		Separator(";"),
	};

	GeometricOperation {
		PrimaryExpression as lhs,
		Operator("*" | "/") as op,
		GenericExpression as rhs,
	};

	ArithmeticOperation {
		PrimaryExpression as lhs,
		Operator("+" | "-") as op,
		GenericExpression as rhs,
	};

	BinaryOperation {
		PrimaryExpression as lhs,
		Operator as op,
		GenericExpression as rhs,
	};

	Join {
		GenericExpression as lhs,
		Separator(","),
		Expression as rhs,
	};

	Group {
		Separator("("),
		Expression as expr,
		Separator(")"),
	};

	Invocation {
		UnaryExpression as expr,
		Separator("("),
		CommaList<GenericExpression> as args,
		Separator(")"),
	};

	UnaryOperation {
		Operator as op,
		UnaryExpression as expr,
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
