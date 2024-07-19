#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AsciiToken {
	LParen,
	RParen,
	LBrace,
	RBrace,
	LCaret,
	RCaret,
	Colon,
	SemiColon,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeywordToken {
	Static,
	Fn,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuiltinToken {
	Println,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
	Ident(String),
	Comment(String),
	String(String),
	Keyword(KeywordToken),
	Builtin(BuiltinToken),
	Ascii(AsciiToken),
}

pub struct Lexer {
	parsing: Vec<char>,
	parsing_idx: usize,
}

impl Lexer {
	pub fn new(src: &str) -> Self {
		Self {
			parsing: src.chars().collect(),
			parsing_idx: 0,
		}
	}

	fn peek(&mut self) -> Option<char> {
		if self.parsing_idx >= self.parsing.len() {
			return None;
		}

		Some(self.parsing[self.parsing_idx])
	}

	fn pop(&mut self) -> Option<char> {
		if self.parsing_idx >= self.parsing.len() {
			return None;
		}

		let c = self.parsing[self.parsing_idx];
		self.parsing_idx += 1;
		Some(c)
	}

	fn pop_until<F: Fn(char) -> bool>(&mut self, predicate: F) -> String {
		let mut str = String::new();
		while let Some(chr) = self.peek() {
			if predicate(chr) {
				break;
			}
			self.pop();
			str.push(chr);
		}
		str
	}

	pub fn lex(&mut self) -> Vec<Token> {
		let mut tokens = Vec::new();

		while let Some(chr) = self.pop() {
			match chr {
				'#' => {
					let content = self.pop_until(|c| c == '\n');
					tokens.push(Token::Comment(content));
				}

				':' => tokens.push(Token::Ascii(AsciiToken::Colon)),
				';' => tokens.push(Token::Ascii(AsciiToken::SemiColon)),
				'(' => tokens.push(Token::Ascii(AsciiToken::LParen)),
				')' => tokens.push(Token::Ascii(AsciiToken::RParen)),
				'<' => tokens.push(Token::Ascii(AsciiToken::LCaret)),
				'>' => tokens.push(Token::Ascii(AsciiToken::RCaret)),
				'{' => tokens.push(Token::Ascii(AsciiToken::LBrace)),
				'}' => tokens.push(Token::Ascii(AsciiToken::RBrace)),

				c if c.is_ascii_alphabetic() => {
					let ident = format!("{c}{}", self.pop_until(|c| !c.is_ascii_alphanumeric()));
					tokens.push(match ident.as_str() {
						"static" => Token::Keyword(KeywordToken::Static),
						"fn" => Token::Keyword(KeywordToken::Fn),

						"println" => Token::Builtin(BuiltinToken::Println),

						_ => Token::Ident(ident),
					});
				}

				'"' => {
					let str = self.pop_until(|c| c == '"');
					self.pop(); // pops the last "
					tokens.push(Token::String(str));
				}

				' ' => {}
				'\n' => {}
				_ => panic!("Don't know what to do with: {chr} : {}", chr as i32),
			}
		}

		tokens
	}
}
