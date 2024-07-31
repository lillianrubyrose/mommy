use maya_bytes::*;

use crate::IOClassfileError;

#[derive(Debug)]
#[repr(u8)]
pub enum IOCpTag {
	// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.7
	Utf8 {
		length: u16,
		bytes: Vec<u8>,
	} = 1,
	Integer {
		bytes: [u8; 4],
	} = 3,
	Float {
		bytes: [u8; 4],
	} = 4,
	// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.5
	// All 8-byte constants take up two entries in the constant_pool table of the class file.
	// If a CONSTANT_Long_info or CONSTANT_Double_info structure is the item in the constant_pool table-
	// at index n, then the next usable item in the pool is located at index n+2.
	// The constant_pool index n+1 must be valid but is considered unusable.
	Long {
		bytes: [u8; 8],
		// high_bytes: [u8; 4],
		// low_bytes: [u8; 4],
	} = 5,
	Double {
		bytes: [u8; 8],
		// high_bytes: [u8; 4],
		// low_bytes: [u8; 4],
	} = 6,
	Class {
		name_index: u16,
	} = 7,
	String {
		utf8_index: u16,
	} = 8,
	FieldRef {
		class_index: u16,
		name_and_ty_index: u16,
	} = 9,
	MethodRef {
		class_index: u16,
		name_and_ty_index: u16,
	} = 10,
	InterfaceMethodRef {
		class_index: u16,
		name_and_ty_index: u16,
	} = 11,
	NameAndType {
		name_index: u16,
		descriptor_index: u16,
	} = 12,
	// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.8
	MethodHandle {
		reference_kind: u8,
		reference_index: u16,
	} = 15,
	// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.9
	MethodType {
		descriptor_index: u16,
	} = 16,
	// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.10
	InvokeDynamic {
		bootstrap_method_attr_index: u16,
		name_and_ty_index: u16,
	} = 18,
	Module {
		name_index: u16,
	} = 19,
	Package {
		name_index: u16,
	} = 20,
}

impl IOCpTag {
	pub fn read<B: BytesReadExt>(buffer: &mut B) -> Result<IOCpTag, IOClassfileError> {
		let tag = buffer.read_u8()?;
		match tag {
			1 => {
				let len = buffer.read_u16()?;
				let mut bytes = Vec::with_capacity(len as usize);
				for _ in 0..len {
					bytes.push(buffer.read_u8()?);
				}

				Ok(IOCpTag::Utf8 { length: len, bytes })
			}
			3 => Ok(IOCpTag::Integer {
				bytes: buffer.read_n_bytes::<4>()?,
			}),
			4 => Ok(IOCpTag::Float {
				bytes: buffer.read_n_bytes::<4>()?,
			}),
			5 => Ok(IOCpTag::Long {
				bytes: buffer.read_n_bytes::<8>()?,
				// high_bytes: buffer.read_n_bytes::<4>()?,
				// low_bytes: buffer.read_n_bytes::<4>()?,
			}),
			6 => Ok(IOCpTag::Double {
				bytes: buffer.read_n_bytes::<8>()?,
				// high_bytes: buffer.read_n_bytes::<4>()?,
				// low_bytes: buffer.read_n_bytes::<4>()?,
			}),
			7 => Ok(IOCpTag::Class {
				name_index: buffer.read_u16()?,
			}),
			8 => Ok(IOCpTag::String {
				utf8_index: buffer.read_u16()?,
			}),
			9 => Ok(IOCpTag::FieldRef {
				class_index: buffer.read_u16()?,
				name_and_ty_index: buffer.read_u16()?,
			}),
			10 => Ok(IOCpTag::MethodRef {
				class_index: buffer.read_u16()?,
				name_and_ty_index: buffer.read_u16()?,
			}),
			11 => Ok(IOCpTag::InterfaceMethodRef {
				class_index: buffer.read_u16()?,
				name_and_ty_index: buffer.read_u16()?,
			}),
			12 => Ok(IOCpTag::NameAndType {
				name_index: buffer.read_u16()?,
				descriptor_index: buffer.read_u16()?,
			}),
			15 => Ok(IOCpTag::MethodHandle {
				reference_kind: buffer.read_u8()?,
				reference_index: buffer.read_u16()?,
			}),
			16 => Ok(IOCpTag::MethodType {
				descriptor_index: buffer.read_u16()?,
			}),
			18 => Ok(IOCpTag::InvokeDynamic {
				bootstrap_method_attr_index: buffer.read_u16()?,
				name_and_ty_index: buffer.read_u16()?,
			}),
			19 => Ok(IOCpTag::Module {
				name_index: buffer.read_u16()?,
			}),
			20 => Ok(IOCpTag::Package {
				name_index: buffer.read_u16()?,
			}),
			_ => unimplemented!("unimplemented tag: {tag}"),
		}
	}

	pub fn id(&self) -> u8 {
		match self {
			IOCpTag::Utf8 { length: _, bytes: _ } => 1,
			IOCpTag::Integer { bytes: _ } => 3,
			IOCpTag::Float { bytes: _ } => 4,
			IOCpTag::Long {
				bytes: _,
				// high_bytes: _,
				// low_bytes: _,
			} => 5,
			IOCpTag::Double {
				bytes: _,
				// high_bytes: _,
				// low_bytes: _,
			} => 6,
			IOCpTag::Class { name_index: _ } => 7,
			IOCpTag::String { utf8_index: _ } => 8,
			IOCpTag::FieldRef {
				class_index: _,
				name_and_ty_index: _,
			} => 9,
			IOCpTag::MethodRef {
				class_index: _,
				name_and_ty_index: _,
			} => 10,
			IOCpTag::InterfaceMethodRef {
				class_index: _,
				name_and_ty_index: _,
			} => 11,
			IOCpTag::NameAndType {
				name_index: _,
				descriptor_index: _,
			} => 12,
			IOCpTag::MethodHandle {
				reference_kind: _,
				reference_index: _,
			} => 15,
			IOCpTag::MethodType { descriptor_index: _ } => 16,
			IOCpTag::InvokeDynamic {
				bootstrap_method_attr_index: _,
				name_and_ty_index: _,
			} => 18,
			Self::Module { name_index: _ } => 19,
			Self::Package { name_index: _ } => 19,
		}
	}

	pub fn write<B: BytesWriteExt>(&self, buffer: &mut B) -> Result<(), IOClassfileError> {
		buffer.write_u8(self.id())?;
		match self {
			IOCpTag::Utf8 { length, bytes } => {
				buffer.write_u16(*length)?;
				buffer.write_all(bytes)?;
			}
			IOCpTag::Integer { bytes } => buffer.write_all(bytes)?,
			IOCpTag::Float { bytes } => buffer.write_all(bytes)?,
			IOCpTag::Long {
				bytes,
				// high_bytes,
				// low_bytes,
			} => {
				buffer.write_all(bytes)?;
				// buffer.write_all(high_bytes)?;
				// buffer.write_all(low_bytes)?;
			}
			IOCpTag::Double {
				bytes,
				// high_bytes,
				// low_bytes,
			} => {
				buffer.write_all(bytes)?;
				// buffer.write_all(high_bytes)?;
				// buffer.write_all(low_bytes)?;
			}
			IOCpTag::Class { name_index } => buffer.write_u16(*name_index)?,
			IOCpTag::String { utf8_index } => buffer.write_u16(*utf8_index)?,
			IOCpTag::FieldRef {
				class_index,
				name_and_ty_index,
			} => {
				buffer.write_u16(*class_index)?;
				buffer.write_u16(*name_and_ty_index)?;
			}
			IOCpTag::MethodRef {
				class_index,
				name_and_ty_index,
			} => {
				buffer.write_u16(*class_index)?;
				buffer.write_u16(*name_and_ty_index)?;
			}
			IOCpTag::InterfaceMethodRef {
				class_index,
				name_and_ty_index,
			} => {
				buffer.write_u16(*class_index)?;
				buffer.write_u16(*name_and_ty_index)?;
			}
			IOCpTag::NameAndType {
				name_index,
				descriptor_index,
			} => {
				buffer.write_u16(*name_index)?;
				buffer.write_u16(*descriptor_index)?;
			}
			IOCpTag::MethodHandle {
				reference_kind,
				reference_index,
			} => {
				buffer.write_u8(*reference_kind)?;
				buffer.write_u16(*reference_index)?;
			}
			IOCpTag::MethodType { descriptor_index } => {
				buffer.write_u16(*descriptor_index)?;
			}
			IOCpTag::InvokeDynamic {
				bootstrap_method_attr_index,
				name_and_ty_index: name_and_type_index,
			} => {
				buffer.write_u16(*bootstrap_method_attr_index)?;
				buffer.write_u16(*name_and_type_index)?;
			}
			IOCpTag::Module { name_index } => {
				buffer.write_u16(*name_index)?;
			}
			IOCpTag::Package { name_index } => {
				buffer.write_u16(*name_index)?;
			}
		}
		Ok(())
	}
}
