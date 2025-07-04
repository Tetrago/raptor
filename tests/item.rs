use raptor::item::Empty;
use raptor::item::File;
use raptor::item::Function;
use raptor::item::Parameter;
use raptor::token::Identifier;
use raptor::token::Lexer;

macro_rules! test {
	($($t:ident $raw:literal => [$($item:expr),+ $(,)?])+) => {
		let mut i = 0;

		$({
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
		File "fn main();" => [
			Function {
				name: Identifier("main"),
				params: Vec::new(),
				stmt: Empty::default().into(),
			}
		]

		File "fn main(a: u32);" => [
			Function {
				name: Identifier("main"),
				params: vec![
					Parameter {
						name: Identifier("a"),
						ty: Identifier("u32")
					}
				],
				stmt: Empty::default().into(),
			}
		]
	};
}
