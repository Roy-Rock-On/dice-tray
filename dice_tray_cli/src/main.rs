mod cli_parser;
mod logger;

use std::io;
use std::io::Write;


fn main() {
	let mut input = String::new();

	loop {
		print!("> ");
		io::stdout().flush().expect("failed to flush stdout");

		input.clear();
		io::stdin()
			.read_line(&mut input)
			.expect("failed to read from stdin");

		let trimmed = input.trim();

		if trimmed.eq_ignore_ascii_case("exit") {
			break;
		}

		println!("You said: {}", trimmed);
	}
}

