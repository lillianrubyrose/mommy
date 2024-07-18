use crate::lex::{AsciiToken, KeywordToken, Token};

// TODO: Bitfield
#[derive(Debug)]
pub enum Modifiers {
	Static,
}

#[derive(Debug)]
pub enum ParsedInstruction {
	Println(String),
}

pub struct Parser {
	tokens: Vec<Token>,
	token_idx: usize,
}

#[derive(Debug)]
pub struct ParsedClass {
	pub name: String,
	pub modifiers: Vec<Modifiers>,
	pub methods: Vec<ParsedMethod>,
}

#[derive(Debug)]
pub struct ParsedMethod {
	pub name: String,
	// TODO: Introduce the repr of types or something I guess? Shouldn't be figuring out the JVM signatures here lol
	pub signature: String,
	pub modifiers: Vec<Modifiers>,
	pub instructions: Vec<ParsedInstruction>,
}

macro_rules! expect_token_value {
	($self:ident, $msg:literal, $match:ident) => {{
		match $self.peek().cloned() {
			Some(Token::$match(i)) => {
				$self.pop();
				i
			}
			_ => panic!($msg),
		}
	}};

	($self:ident, $msg:literal, $a:ident($b:ident::$c:ident)) => {{
		match $self.peek() {
			Some(Token::$a($b::$c(i))) => i,
			_ => panic!($msg),
		}
	}};
}

impl Parser {
	pub fn new(tokens: Vec<Token>) -> Self {
		Self {
			tokens,
			token_idx: 0,
		}
	}

	pub fn parse(&mut self) -> ParsedClass {
		let mut methods = Vec::new();
		while let Some(t) = self.peek() {
			match t {
				Token::Ident(_) => {
					self.pop();
				}
				Token::Comment(_) => {
					self.pop();
				}
				Token::String(_) => {
					self.pop();
				}
				Token::Keyword(t) => match t {
					KeywordToken::Static => {
						methods.push(self.parse_method())
					}
					KeywordToken::Fn => todo!(),
				},
				Token::Builtin(_) => {
					self.pop();
				}
				Token::Ascii(_) => {
					self.pop();
				}
			}
		}

		ParsedClass {
			name: String::from("Main"),
			modifiers: vec![],
			methods,
		}
	}

	fn peek(&mut self) -> Option<&Token> {
		self.tokens.get(self.token_idx)
	}

	fn pop(&mut self) -> Option<&Token> {
		if self.token_idx >= self.tokens.len() {
			return None;
		}
		let v = self.tokens.get(self.token_idx);
		self.token_idx += 1;
		v
	}

	fn collect_modifiers(&mut self) -> Vec<Modifiers> {
		let mut v = vec![];
		while let Some(t) = self.peek() {
			if let Token::Keyword(t) = t {
				match t {
					KeywordToken::Static => {
						self.pop();
						v.push(Modifiers::Static);
					}
					KeywordToken::Fn => break,
				}
			}
		}
		v
	}

	fn expect_token(&mut self, msg: &str, ty: Token) {
		if let Some(t) = self.pop() {
			if t != &ty {
				panic!("t={t:?} {msg}");
			}
		} else {
			panic!("{msg}");
		}
	}

	fn parse_args(&mut self) {
		while let Some(Token::Ident(_)) = self.peek() {
			self.pop();
			self.expect_token(
				"expected ':'",
				Token::Ascii(AsciiToken::Colon),
			);
			expect_token_value!(self, "expected ident", Ident);
			// TODO: support multiple args
			self.expect_token(
				"expected ')'",
				Token::Ascii(AsciiToken::RParen),
			);
		}
	}

	fn parse_method(&mut self) -> ParsedMethod {
		let modifiers = self.collect_modifiers();
		self.expect_token(
			"expected 'fn'",
			Token::Keyword(KeywordToken::Fn),
		);
		let name =
			expect_token_value!(self, "expected ident", Ident).clone();
		self.expect_token(
			"expected '('",
			Token::Ascii(AsciiToken::LParen),
		);
		dbg!(&name);

		self.parse_args();
		self.expect_token(
			"expected '{'",
			Token::Ascii(AsciiToken::LBrace),
		);
		let mut instructions = Vec::new();

		let Some(next) = self.pop() else {
			panic!("e");
		};
		match next {
			Token::Ident(_) => todo!(),
			Token::Comment(_) => todo!(),
			Token::String(_) => todo!(),
			Token::Keyword(_) => todo!(),
			Token::Builtin(t) => match t {
				crate::lex::BuiltinToken::Println => {
					self.expect_token(
						"expected '('",
						Token::Ascii(AsciiToken::LParen),
					);
					instructions.push(ParsedInstruction::Println(
						expect_token_value!(
							self,
							"expected string",
							String
						),
					));
					self.expect_token(
						"expected ')'",
						Token::Ascii(AsciiToken::RParen),
					);
					self.expect_token(
						"expected ';'",
						Token::Ascii(AsciiToken::SemiColon),
					);
				}
			},
			Token::Ascii(_) => todo!(),
		};
		self.expect_token(
			"expected '}'",
			Token::Ascii(AsciiToken::RBrace),
		);

		// getstatic
		// ldc (constant pool)
		// invokevirtual
		// return

		ParsedMethod {
			name,
			signature: String::from("()V"),
			modifiers,
			instructions,
		}
	}
}
