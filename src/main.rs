use std::io::Write;

#[macro_use]
extern crate lazy_static;

mod scanner;
mod lexer_types;
mod parser;
mod expr;


fn main() {
	let mut scanner = scanner::scanner::new(input("Enter lex: ").unwrap());
	scanner.scan_tokens();
	println!("{:#?}", scanner.get_tokens());
}

fn input(prompt: &str) -> std::io::Result<String> {
	print!("{}", prompt);
	std::io::stdout().flush()?;
	let mut result = String::new();
	std::io::stdin().read_line(&mut result)?;
	Ok(result.trim().to_string())
}