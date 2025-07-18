use raptor::item::Empty;
use raptor::item::File;
use raptor::item::Function;
use raptor::item::Parameter;
use raptor::lexer::Identifier;
use raptor::lexer::Lexer;
use raptor::prelude::*;

macro_rules! test {
	($($t:ident($($src:ident =)? $raw:literal) => [$($item:expr),+ $(,)?])+) => {
		let mut i = 0;

		$({
			$(let $src = $raw;)?
			i += 1;

			let file = <$t>::parse(Lexer::new($raw));
			let expected = File {
				items: vec![$($item.into()),+]
			};

			assert_eq!(file, expected, "Condition {i} failed.");
		})+
	};
}

#[test]
fn function() {
	test! {
		File(src = "fn main();") => [
			Function {
				name: src.span(3..7).wrap(Identifier("main")),
				params: Vec::new(),
				stmt: Empty::default().into(),
			}
		]

		File(src = "fn main(a: u32, b: u32);") => [
			Function {
				name: src.span(3..7).wrap(Identifier("main")),
				params: vec![
					Parameter {
						name: src.span(8..9).wrap(Identifier("a")),
						ty: src.span(11..14).wrap(Identifier("u32")),
					},
					Parameter {
						name: src.span(16..17).wrap(Identifier("b")),
						ty: src.span(19..22).wrap(Identifier("u32")),
					}
				],
				stmt: Empty::default().into(),
			}
		]
	};
}
