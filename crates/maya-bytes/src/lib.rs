#![feature(seek_stream_len)]

mod macros;

use std::io::{Read, Seek, Write};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BytesError {
	#[error("Not enough data left in the buffer")]
	NotEnoughData,
	#[error("IO Error: {0}")]
	IO(#[from] std::io::Error),
}

pub trait BytesExt: Read + Write + Seek {
	define_integral_rw!(i8, 1);
	define_integral_rw!(u8, 1);

	define_integral_rw!(i16, 2);
	define_integral_rw!(u16, 2);

	define_integral_rw!(i32, 4);
	define_integral_rw!(u32, 4);

	define_integral_rw!(i64, 8);
	define_integral_rw!(u64, 8);

	define_write!(f32);
	define_write!(f64);

	fn len_check(&mut self, needed: u64) -> Result<(), BytesError> {
		let len = self.stream_len()?;
		if len < self.stream_position()? + needed {
			return Err(BytesError::NotEnoughData);
		}

		Ok(())
	}

	fn read_n_bytes<const N: usize>(
		&mut self,
	) -> Result<[u8; N], BytesError> {
		self.len_check(N as u64)?;

		let mut bytes = [0u8; N];
		self.read_exact(&mut bytes)?;
		Ok(bytes)
	}

	fn read_n_bytes_vec(
		&mut self,
		amount: usize,
	) -> Result<Vec<u8>, BytesError> {
		self.len_check(amount as u64)?;

		let mut bytes = vec![0; amount];
		self.read_exact(&mut bytes)?;
		Ok(bytes)
	}

	fn read_f32(&mut self) -> Result<f32, BytesError> {
		self.len_check(4)?;

		let v = self.read_u32()?;
		Ok(f32::from_bits(v))
	}

	fn read_f64(&mut self) -> Result<f64, BytesError> {
		self.len_check(8)?;

		let v = self.read_u64()?;
		Ok(f64::from_bits(v))
	}
}

impl<R: Read + Write + Seek> BytesExt for R {}

#[cfg(test)]
mod tests {
	use std::io::Cursor;

	use super::BytesExt;

	macro_rules! define_test {
		($ty:ty) => {
			paste::item! {
				#[test]
				fn [<$ty>]() {
					const VALUES: [$ty; 2] = [$ty::MAX, $ty::MIN];

					for v in VALUES {
						let mut buffer: Cursor<Vec<u8>> =
							Cursor::new(Vec::new());
						buffer.[<write_$ty>](v).expect("Error when writing");
						buffer.set_position(0);
						assert_eq!(
							v,
							buffer.[<read_$ty>]().expect("Error when reading")
						);
					}
				}
			}
		};
	}

	define_test!(i8);
	define_test!(u8);
	define_test!(i16);
	define_test!(u16);
	define_test!(i32);
	define_test!(u32);
	define_test!(i64);
	define_test!(u64);
	define_test!(f32);
	define_test!(f64);
}
