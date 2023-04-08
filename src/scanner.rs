use crate::lexer_types;
use std::collections::HashMap;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, lexer_types::TokenType> = {
		hashmap![
			"and" => lexer_types::TokenType::AND,
			"class" => lexer_types::TokenType::CLASS,
			"else" => lexer_types::TokenType::ELSE,
			"false" => lexer_types::TokenType::FALSE,
			"for" => lexer_types::TokenType::FOR,
			"fun" => lexer_types::TokenType::FUN,
			"if" => lexer_types::TokenType::IF,
			"nil" => lexer_types::TokenType::NIL,
			"or" => lexer_types::TokenType::OR,
			"print" => lexer_types::TokenType::PRINT,
			"return" => lexer_types::TokenType::RETURN,
			"super" => lexer_types::TokenType::SUPER,
			"this" => lexer_types::TokenType::SUPER,
			"true" => lexer_types::TokenType::TRUE,
			"var" => lexer_types::TokenType::VAR,
			"while" => lexer_types::TokenType::WHILE
		]
    };
}

pub struct scanner {
	source: String,
	tokens: Vec<lexer_types::Token>,
	start: usize,
	current: usize,
	line: i32
}

impl scanner {
	pub fn new(source: String) -> scanner {
		scanner { source: source, start: 0, current: 0, line: 1, tokens: vec![] }
	}
	pub fn scan_tokens(&mut self) {
		while !self.is_at_end() {
			self.start = self.current;
			self.scan_token();
		}
		self.tokens.push(lexer_types::Token::new(lexer_types::TokenType::EOF, "".to_string(), self.line))
	}

	fn scan_token(&mut self) {
		let c: char = self.advance();
		match c {
			'(' => self.add_token(lexer_types::TokenType::LEFT_PAREN),
			')' => self.add_token(lexer_types::TokenType::RIGHT_PAREN),
			'{' => self.add_token(lexer_types::TokenType::LEFT_BRACE),
			'}' => self.add_token(lexer_types::TokenType::RIGHT_BRACE),
			',' => self.add_token(lexer_types::TokenType::COMMA),
			'.' => self.add_token(lexer_types::TokenType::DOT),
			'-' => self.add_token(lexer_types::TokenType::MINUS),
			'+' => self.add_token(lexer_types::TokenType::PLUS),
			';' => self.add_token(lexer_types::TokenType::SEMICOLON),
			'*' => self.add_token(lexer_types::TokenType::STAR),
			'=' => {
				if self.peek() == '=' {
					self.add_token(lexer_types::TokenType::EQUAL_EQUAL);
				} else {
					self.add_token(lexer_types::TokenType::EQUAL);
				}
			},
			'"' => {
				self.string();
			}
			'/' => {
				if self.match_('/') {
					while self.peek() != '\n' && !self.is_at_end() { self.advance(); }
				} else {
					self.add_token(lexer_types::TokenType::SLASH);
				}
			},
			' ' => {},
			'\r' => {},
			't' => {},
			'\n' => self.line += 1,
			_ => {
				if self.is_digit(c) {
					self.number();
				} else if self.is_alpha(c) {
					self.identifier();
				} else {
					panic!("Unexpected character at line {}", self.line);
				}
			}
		}
	}

	fn identifier(&mut self) {
		while self.is_alpha_numerical(self.peek()) { self.advance(); }
		let text = self.source[(self.start)..(self.current)].to_string();
		let mut type_ = KEYWORDS.get_key_value(text.as_str());
		if type_.is_none() {
			type_ = Some((&"", &lexer_types::TokenType::IDENTIFIER));
		}
		self.add_token(type_.unwrap().1.to_owned()); 
	}

	fn number(&mut self) {
		while self.is_digit(self.peek()) { self.advance(); }
		if self.peek() == '.' && self.is_digit(self.peek_next()) {
			self.advance();
			while self.is_digit(self.peek()) { self.advance(); }
		}

		let value: f64 = str::parse(&self.source[(self.start)..(self.current)]).unwrap();
		self.add_token(lexer_types::TokenType::DOUBLE(value))
	}

	fn is_digit(&self, c: char) -> bool {
		c >= '0' && c <= '9'
	}

	fn peek_next(&self) -> char {
		if self.current + 1usize >= self.source.len() { return '\0' }
		self.source.chars().nth((self.current - 1) as usize).unwrap()
	}

	fn string(&mut self) {
		while self.peek() != '"' && !self.is_at_end() {
			if self.peek() == '\n' { self.line += 1 }
			self.advance();
		}

		if self.is_at_end() {
			panic!("Unterminated string literal");
		}

		self.advance();

		let value: String = self.source[(self.start + 1usize)..(self.current - 1usize)].to_string();
		self.add_token(lexer_types::TokenType::STRING(value));
	}

	fn peek(&self) -> char {
		if self.is_at_end() { return '\0' }
		self.source.chars().nth((self.current) as usize).unwrap()
	}

	fn advance(&mut self) -> char {
		self.current += 1;
		self.source.chars().nth((self.current - 1) as usize).unwrap()
	}

	fn add_token(&mut self, type_: lexer_types::TokenType) {
		let text: String = self.source[usize::from(self.start)..self.current.into()].to_string();
		self.tokens.push(lexer_types::Token::new(type_, text, self.line))
	}

	fn is_alpha(&self, c: char) -> bool{
		(c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
	}

	fn is_alpha_numerical(&self, c: char) -> bool {
		if self.is_alpha(c) {
			return true
		} else if str::parse::<i32>(&format!("{}", c)).is_ok() {
			return true
		}
		false
	}

	fn is_at_end(&self) -> bool {
		self.current >= self.source.len()
	}

	pub fn get_tokens(&self) -> Vec<lexer_types::Token> {
		self.tokens.clone()
	}

	fn match_(&mut self, expected: char) -> bool {
		if self.is_at_end() { return false }
		if self.source.chars().nth(self.current).unwrap() != expected { return false }
		self.current += 1;
		return true;
	}
}