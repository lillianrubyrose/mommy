#[macro_export]
macro_rules! define_write {
	($ty:ty) => {
		paste::item! {
		fn [<write_$ty>](
			&mut self,
			value: $ty,
		) -> Result<(), BytesError> {
			let bytes = value.to_be_bytes();
			self.write_all(&bytes)?;
			Ok(())
		}
		}
	};
}

#[macro_export]
macro_rules! define_integral_rw {
	($ty:ty, $bytes:expr) => {
		paste::item! {
		fn [<read_$ty>](
			&mut self,
		) -> Result<$ty, BytesError> {
			self.len_check($bytes)?;

			let mut data = [0u8; $bytes];
			self.read_exact(&mut data)?;
			Ok($ty::from_be_bytes(data))
		}
		}

		define_write!($ty);
	};
}
