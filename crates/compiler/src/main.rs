#![allow(dead_code)]

use lex::Lexer;
use parse::Parser;

mod lex;
mod parse;

fn main() {
	let src = include_str!("../assets/test.mommy");
	let mut lexer = Lexer::new(src);
	let tokens = lexer.lex();
	println!("{:?}", &tokens);

	let mut parser = Parser::new(tokens);
	println!("{:?}", parser.parse());
}
