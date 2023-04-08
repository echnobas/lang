use crate::lexer_types::*;
use crate::scanner;
use crate::expr;

pub struct Parser {
	tokens: Vec<Token>,
	current: i32,
	in_func_dec: bool
}

impl Default for Parser {
	fn default() -> Parser {
		Parser {
			tokens: Vec::new(),
			current: 0,
			in_func_dec: false
		}
	}
}
pub fn parse(tokens: Vec<Token>) {
	let mut p = Parser {
		tokens,
		..Default::default()
	};

	let stmts_or_err = p.parse();
	match stmts_or_err {
		Ok(v) => {
			if !p.is_at_end() {
				let tok = &p.tokens[p.current];
				Err(format!(
					"unexpected token of type {:?} at line = {}, col = {}",
					tok.ty, tok.line, tok.col
				))
			}
		}
	}
}
