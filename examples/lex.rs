use std::process::ExitCode;

use raptor::lexer::Lexer;
use raptor::lexer::Token;

fn usage() {
	println!(
		"\
Example utility for printing the token representation of a text input.

Usage: cargo run --example lex [-- [-h]] <file>

Options:
  -h, --help  Display this usage information."
	);
}

fn main() -> ExitCode {
	let args = std::env::args().collect::<Vec<_>>();

	if args.len() != 2 {
		usage();

		return if args.len() == 1 {
			ExitCode::SUCCESS
		} else {
			ExitCode::FAILURE
		};
	}

	if args[1] == "-h" || args[1] == "--help" {
		usage();
		return ExitCode::SUCCESS;
	}

	if let Ok(contents) = std::fs::read_to_string(&args[1]) {
		let lexer = Lexer::new(&contents);
		let tokens = lexer
			.filter(|token| !matches!(token.value, Token::Comment(_) | Token::Whitespace(_)))
			.collect::<Vec<_>>();

		println!("{tokens:#?}");
		ExitCode::SUCCESS
	} else {
		ExitCode::FAILURE
	}
}
