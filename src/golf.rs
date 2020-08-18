use std::collections::{HashMap, HashSet};
use std::mem;

#[derive(PartialEq, Eq, Clone, Copy)]
enum TokenType {
	SpecialCharacter,
	Number,
	Identifier,
}

struct Golfifyer<'a> {
	output: String,
	temp_string: String,
	id_counter: usize,
	id_blacklist: &'a HashSet<String>,
	id_map: HashMap<String, usize>,
	chars: std::str::Chars<'a>,
	previous_token: TokenType,
}

impl Golfifyer<'_> {
	fn peek_char(&self) -> Option<char> {
		self.chars.clone().next()
	}

	fn prepare_for_identifier(&mut self) {
		if self.previous_token == TokenType::Identifier {
			self.output.push(' ');
		}
	}

	fn prepare_for_number(&mut self) {
		if self.previous_token == TokenType::Identifier || 
			self.previous_token == TokenType::Number
		{
			self.output.push(' ');
		}
	}

	fn read_identifier(&mut self) {
		// Avoid allocating so many things
		let mut identifier = mem::replace(&mut self.temp_string, String::new());
		identifier.clear();

		identifier.push(
			self.chars.next()
				.expect("Don't call read_identifier when the character isn't an identifier!!!!")
		);

		while let Some(c) = self.peek_char() {
			if c.is_alphabetic() || c.is_digit(10) || c == '_' {
				identifier.push(c);
				self.chars.next();
			} else {
				break;
			}
		}

		if self.id_blacklist.contains(&identifier) {
			// It can't be shortened! Just use this identifier then
			self.prepare_for_identifier();
			self.output.push_str(&identifier);
		} else {
			self.prepare_for_identifier();
			match self.id_map.get(&identifier) {
				Some(&name_id) => {
					let result = generate_id_and_push(&mut self.output, name_id, self.id_blacklist);
					// The id should be valid
					assert!(result);
				},
				None => {
					while !generate_id_and_push(&mut self.output, self.id_counter, self.id_blacklist) {
						self.id_counter += 1;
					}
					self.id_map.insert(identifier.clone(), self.id_counter);
					self.id_counter += 1;
				},
			};
		}

		self.previous_token = TokenType::Identifier;
		self.temp_string = identifier;
	}

	fn read_number(&mut self) {
		use std::fmt::Write;

		let mut number = 0;
		let mut base = 10;

		let first = self.chars.next().expect("Don't call read_number without a number first");

		if first == '0' && self.peek_char() == Some('x') {
			base = 16;
			self.chars.next();
		} else {
			number += first.to_digit(10)
				.expect("Don't call read_number without a number first") as i128;
		}

		while let Some(c) = self.peek_char() {
			if let Some(digit) = c.to_digit(base) {
				number = match number.checked_mul(base as i128) {
					Some(num) => num,
					None => {
						println!("WARNING: A number is too big");
						0
					},
				};
				number = match number.checked_add(digit as i128) {
					Some(num) => num,
					None => {
						println!("WARNING: A number is too big");
						0
					},
				};

				self.chars.next();
			} else {
				break;
			}
		}

		self.prepare_for_number();

		// Find which notation is the most compact
		let mut n_decimal_digits = 0;
		let mut cnt = 1;
		while cnt < number {
			cnt *= 10;
			n_decimal_digits += 1;
		}
		let mut n_hex_digits = 2; // '0x' takes two spots already
		cnt = 1;
		while cnt < number {
			cnt *= 16;
			n_hex_digits += 1;
		}

		if n_decimal_digits > n_hex_digits {
			write!(&mut self.output, "{:#X}", number).unwrap();
		} else {
			write!(&mut self.output, "{}", number).unwrap();
		}

		self.previous_token = TokenType::Number;
	}

	fn read_string_literal(&mut self) {
		assert_eq!(self.chars.next(), Some('"'));
		self.output.push('"');

		while let Some(c) = self.chars.next() {
			self.output.push(c);
			if c == '\\' {
				// Don't do anything special with the next character
				if let Some(c) = self.chars.next() {
					self.output.push(c);
				}
			} else if c == '"' {
				break;
			}
		}

		self.previous_token = TokenType::SpecialCharacter;
	}
}

pub fn golfify_code(code: &str, id_blacklist: &HashSet<String>) -> String {
	let mut golfifyer = Golfifyer {
		output: String::new(),
		id_map: HashMap::new(),
		id_counter: 0,
		temp_string: String::new(),
		id_blacklist: id_blacklist,
		chars: code.chars(),
		previous_token: TokenType::SpecialCharacter,
	};

	while let Some(c) = golfifyer.peek_char() {
		if c.is_alphabetic() || c == '_' {
			golfifyer.read_identifier();
		} else if c.is_digit(10) {
			golfifyer.read_number();
		} else if c == '"' {
			golfifyer.read_string_literal();
		} else if c == '#' {
			// Preprocessor directive. Just skip until it's over
			while let Some(c) = golfifyer.chars.next() {
				golfifyer.output.push(c);
				if c == '\n' {
					break;
				}
			}
			// The previous character is a \n, normally whitespace characters are not inserted
			// but here we do not have a choice.
			golfifyer.previous_token = TokenType::SpecialCharacter;
		} else {
			if !c.is_whitespace() {
				golfifyer.output.push(c);
				golfifyer.previous_token = TokenType::SpecialCharacter;
			}

			golfifyer.chars.next();
		}
	}

	golfifyer.output
}

fn generate_id_and_push(buffer: &mut String, mut counter: usize, blacklist: &HashSet<String>) -> bool {
	const FIRST: &[u8] = b"_abcdefghijklmnopqrstuvwxyzABCDEFGHILKLMNOPQRSTUVWXYZ";
	const MORE: &[u8] = b"_abcdefghijklmnopqrstuvwxyzABCDEFGHILKLMNOPQRSTUVWXYZ0123456789";

	let start = buffer.len();
	buffer.push(FIRST[counter % FIRST.len()].into());
	counter /= FIRST.len();

	while counter > 0 {
		buffer.push(MORE[counter % MORE.len()].into());
		counter /= MORE.len();
	}

	if blacklist.contains(&buffer[start..]) {
		buffer.truncate(start);
		return false;
	}

	true
}
