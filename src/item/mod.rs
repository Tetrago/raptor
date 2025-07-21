use crate::group;
use crate::lexer::Parseable;
use crate::lexer::TokenStream;
use crate::lexer::*;
use crate::prelude::*;

pub mod list;
mod macros;

use list::*;
use macros::*;

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
		lhs: SecondaryExpression,
		op: {Operator},
		rhs: (TertiaryExpression),
	}

	ArithmeticOperation {
		lhs: SecondaryExpression,
		op: {Operator},
		rhs: (TertiaryExpression),
	}

	BinaryOperation {
		lhs: SecondaryExpression,
		op: {Operator},
		rhs: (TertiaryExpression),
	}

	Join {
		lhs: TertiaryExpression,
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
		expr: PrimaryExpression,
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
	PrimaryExpression {
		Group,
		IdentifierExpression,
		LiteralExpression,
	}

	SecondaryExpression {
		MonoOperation,
		Invocation,
		PrimaryExpression,
	}

	TertiaryExpression {
		GeometricOperation,
		ArithmeticOperation,
		BinaryOperation,
		SecondaryExpression,
	}

	Expression {
		Join,
		TertiaryExpression,
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
		SecondaryExpression as lhs,
		Operator("*" | "/") as op,
		TertiaryExpression as rhs,
	};

	ArithmeticOperation {
		SecondaryExpression as lhs,
		Operator("+" | "-") as op,
		TertiaryExpression as rhs,
	};

	BinaryOperation {
		SecondaryExpression as lhs,
		Operator as op,
		TertiaryExpression as rhs,
	};

	Join {
		TertiaryExpression as lhs,
		Separator(","),
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
		PrimaryExpression as expr,
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
