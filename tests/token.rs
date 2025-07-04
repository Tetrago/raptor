use raptor::token::*;

macro_rules! test {
	($name:ident, $text:literal => { $($expr:expr),+ $(,)? }) => {
		#[test]
		fn $name() {
			use ::raptor::token::Lexer;
			use ::raptor::token::Token;

			let contents = $text.trim();
			let parser = Lexer::new(&contents);

			let tokens = [$($expr.into()),+].iter().scan(0, |state: &mut usize, token: &Token<'_>| {
				let span = Span {
					src: &contents,
					from: *state,
					to: *state + token.slice().len()
				};

				*state += token.slice().len();
				Some((span, *token))
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
