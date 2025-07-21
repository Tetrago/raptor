use raptor::lexer::*;
use raptor::prelude::*;

macro_rules! test {
	($name:ident, $text:literal => { $($expr:expr),+ $(,)? }) => {
		#[test]
		fn $name() {
			use ::raptor::lexer::Lexer;
			use ::raptor::lexer::Token;

			let contents = $text.trim();
			let parser = Lexer::new(&contents);

			let tokens = [$($expr.into()),+].iter().scan(0, |state: &mut usize, token: &Token<'_>| {
				let span = contents.span(*state..*state + token.as_str().len());
				*state += token.as_str().len();
				Some(span.wrap(*token))
			}).collect::<Vec<_>>();

			parser.zip(tokens.iter()).for_each(|(parsed, expected)| {
				assert_eq!(parsed, *expected)
			});
		}
	};
	($name:ident, $text:literal => $expr:expr) => {
		test!($name, $text => { $expr });
	};
}

test!(empty_main_fn, r"fn main() {}" => {
	Identifier("fn"),
	Whitespace(" "),
	Identifier("main"),
	Separator("("),
	Separator(")"),
	Whitespace(" "),
	Separator("{"),
	Separator("}"),
});

test!(param_main_fn, r"fn main(a: u32, b: u32,) {}" => {
	Identifier("fn"),
	Whitespace(" "),
	Identifier("main"),
	Separator("("),
	Identifier("a"),
	Operator(":"),
	Whitespace(" "),
	Identifier("u32"),
	Separator(","),
	Whitespace(" "),
	Identifier("b"),
	Operator(":"),
	Whitespace(" "),
	Identifier("u32"),
	Separator(","),
	Separator(")"),
	Whitespace(" "),
	Separator("{"),
	Separator("}"),
});

test!(field_inequality, r"obj.field != value" => {
	Identifier("obj"),
	Operator("."),
	Identifier("field"),
	Whitespace(" "),
	Operator("!="),
	Whitespace(" "),
	Identifier("value"),
});

test!(backtick_identifier, r"let `backtick field` = 3;" => {
	Identifier("let"),
	Whitespace(" "),
	Identifier("`backtick field`"),
	Whitespace(" "),
	Operator("="),
	Whitespace(" "),
	Literal("3"),
	Separator(";"),
});

test!(integer_literal, r"000" => Literal("000"));
test!(hex_literal, r"0x0" => Literal("0x0"));
test!(octal_literal, r"0o0" => Literal("0o0"));
test!(binary_literal, r"0b0" => Literal("0b0"));
test!(zero_decimal, r".0" => Literal(".0"));
test!(decimal_literal, r"0.05" => Literal("0.05"));
test!(invalid_decimal, r"0." => { Literal("0"), Operator(".") });

test!(comment_eof, r"// tmp" => Comment("// tmp"));
test!(comment_newline, "// tmp\n" => Comment("// tmp"));
test!(multiline_comment, r"/* tmp */ " => { Comment("/* tmp */"), Whitespace(" ") });
