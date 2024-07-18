#![feature(portable_simd)]

use std::{simd::u8x16, string::FromUtf8Error};

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum MUTFError {
	#[error("nullbyte in input")]
	NullByteInInput,
	#[error("{0} byte codepoint: input length too small")]
	CodepointBadInputLength(u8),
	#[error("UTF-8 Error: {0}")]
	FromUTF8Err(#[from] FromUtf8Error),
	#[error("Input has wrong encoding")]
	InvalidEncoding,
}

pub fn encode(string: &str) -> Vec<u8> {
	let mut bytes: Vec<u8> = vec![];

	for c in string.chars().map(|c| c as u32) {
		match c {
			// nullbytes are handled weird
			0 => bytes.extend([0xC0, 0x80]),

			// valid ascii
			c @ 0..=0x7F => bytes.push(c as u8),

			// 2 byte encoding
			c @ 0..=0x7FF => bytes.extend([
				0xC0 | 0x1F & (c >> 0x06) as u8,
				0x80 | (0x3F & c) as u8,
			]),

			// 3 byte encoding
			c @ 0..=0x7FFF => bytes.extend([
				0xE0 | 0x0F & (c >> 0x0C) as u8,
				0x80 | 0x3F & (c >> 0x06) as u8,
				0x80 | (0x3F & c) as u8,
			]),

			// 6 byte encoding
			_ => bytes.extend([
				0xED,
				0xA0 | (c >> 0x10) as u8 & 0x0F,
				0x80 | (c >> 0x0A) as u8 & 0x3F,
				0xED,
				0xB0 | (c >> 0x06) as u8 & 0x0F,
				0x80 | (c & 0x3F) as u8,
			]),
		}
	}

	bytes
}

pub fn decode(input: &[u8]) -> Result<String, MUTFError> {
	let mut output: Vec<u8> = vec![];
	let len = input.len();
	let mut idx = 0;

	while idx + 16 <= len {
		let chunk = u8x16::from_slice(&input[idx..idx + 16]);
		let is_ascii = chunk.lt(&u8x16::splat(0x80));
		if is_ascii {
			output.extend_from_slice(&input[idx..idx + 16]);
			idx += 16;
		} else {
			break;
		}
	}

	while idx < len {
		let b = input[idx];
		idx += 1;

		match b {
			0x0 => return Err(MUTFError::NullByteInInput),
			// valid ascii
			b if b < 0x80 => output.push(b),
			// 2 byte encoding
			b if (b & 0xE0) == 0xC0 => {
				if idx >= len {
					return Err(MUTFError::CodepointBadInputLength(2));
				}

				let b2 = input[idx];
				idx += 1;

				if b != 0xC0 || b2 != 0x80 {
					output.extend([b, b2]);
				} else {
					output.push(0);
				}
			}

			// 3 byte encoding
			b if (b & 0xF0) == 0xE0 => {
				if idx + 1 >= len {
					return Err(MUTFError::CodepointBadInputLength(3));
				}

				let b2 = input[idx];
				let b3 = input[idx + 1];
				idx += 2;

				// check for 6 byte encoding
				if idx + 2 < len && b == 0xED && (b2 & 0xF0) == 0xA0 {
					let b4 = input[idx];
					let b5 = input[idx + 1];
					let b6 = input[idx + 2];

					// its 6 byte encoding!
					if b4 == 0xED && (b5 & 0xF0) == 0xB0 {
						idx += 3;

						let mut bits: u32 = ((b2 as u32 & 0x0F) + 1) << 16;
						bits += (b3 as u32) & 0x3F << 10;
						bits += (b5 as u32) & 0x0F << 6;
						bits += (b6 as u32) & 0x3F;

						output.push(0xF0 + ((bits >> 18) & 0x07) as u8);
						output.push(0x80 + ((bits >> 12) & 0x3F) as u8);
						output.push(0x80 + ((bits >> 6) & 0x3F) as u8);
						output.push(0x80 + (bits & 0x3F) as u8);
						continue;
					}
				}

				output.extend([b, b2, b3]);
			}

			_ => return Err(MUTFError::InvalidEncoding),
		}
	}

	Ok(String::from_utf8(output)?)
}

#[cfg(test)]
mod tests {
	use crate::*;

	#[test]
	fn basic() {
		const STR: &str = "hello world";
		let encoded = encode(STR);
		let decoded = decode(&encoded);
		assert!(decoded.is_ok());
		assert_eq!(STR, decoded.unwrap());
	}

	#[test]
	fn two_byte() {
		// THIS FILE HAS TO BE UTF-8 OR IT DIES
		const STR: &str = "Œ";
		let encoded = encode(STR);
		let decoded = decode(&encoded);
		assert!(decoded.is_ok());
		assert_eq!(STR, decoded.unwrap());
	}

	#[test]
	fn three_byte() {
		// THIS FILE HAS TO BE UTF-8 OR IT DIES
		const STR: &str = "•";
		let encoded = encode(STR);
		let decoded = decode(&encoded);
		assert!(decoded.is_ok());
		assert_eq!(STR, decoded.unwrap());
	}

	#[test]
	fn six_byte() {
		// THIS FILE HAS TO BE UTF-8 OR IT DIES
		const STR: &str = "〰";
		let encoded = encode(STR);
		let decoded = decode(&encoded);
		assert!(decoded.is_ok());
		assert_eq!(STR, decoded.unwrap());
	}

	#[test]
	fn complex_string() {
		const STR: &str = "Hello World! Œ and 〰 and • plus more ascii!";
		let encoded = encode(STR);
		let decoded = decode(&encoded);
		assert!(decoded.is_ok());
		assert_eq!(STR, decoded.unwrap());
	}

	#[test]
	fn decode_null_byte_in_input() {
		let input = b"\x00";
		let result = decode(input);
		assert!(matches!(result, Err(MUTFError::NullByteInInput)));
	}

	#[test]
	fn decode_invalid_encoding() {
		let input = b"\x80";
		let result = decode(input);
		assert!(matches!(result, Err(MUTFError::InvalidEncoding)));
	}

	#[test]
	fn decode_codepoint_bad_input_length() {
		let input = b"\xC2";
		let result = decode(input);
		assert!(matches!(
			result,
			Err(MUTFError::CodepointBadInputLength(2))
		));
	}
}
