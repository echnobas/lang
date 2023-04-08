#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum TokenType {
	// Single-character tokens.
	LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
	COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

	BANG, BANG_EQUAL,
	EQUAL, EQUAL_EQUAL,
	GREATER, GREATER_EQUAL,
	LESS, LESS_EQUAL,
  
	// Literals.
	IDENTIFIER, STRING(String), DOUBLE(f64), NUMBER(i32),
  
	// Keywords.
	AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
	PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,
  
	EOF
}

#[derive(Debug, Clone)]
pub struct Token {
	type_: TokenType,
	lexeme: String,
	line: i32
}

impl Token {
	pub fn new(type_: TokenType, lexeme: String, line: i32) -> Token {
		Token { type_, lexeme, line }
	}

	pub fn to_string(&self) -> String {
		format!("{:?} {} {}", self.type_, self.lexeme, self.line)
	}
}